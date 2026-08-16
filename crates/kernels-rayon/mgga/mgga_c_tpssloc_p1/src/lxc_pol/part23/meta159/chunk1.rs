//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 739/1527 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk739(t28: f64, t3672: f64, t517: f64, t5966: f64, t6312: f64, t157: f64, t6311: f64, zeta_threshold: f64) -> f64 {
    let t29 = t28 <= zeta_threshold;
    let t6318 = piecewise3(t29, 0.0_f64, 4.0_f64 / 9.0_f64 * t3672 * t6312 + 4.0_f64 / 3.0_f64 * t517 * t5966);
    let t6320 = (t6311 + t6318) * t157;
    t6320
}
