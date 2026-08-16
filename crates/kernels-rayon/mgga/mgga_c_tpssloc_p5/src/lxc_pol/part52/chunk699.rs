//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 52 (v4rho2sigma2_8) CSE chunk 699/1400 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part52_v4rho2sigma2_8_chunk699(t1862: f64, t33: f64, t2240: f64, t645: f64, t79: f64, t72: f64) -> (f64, f64, f64) {
    let t6489 = t33 * t1862;
    let t6490 = t2240 * t6489;
    let t6491 = t79 * t645;
    let t6492 = t72 * t6491;
    (t6489, t6490, t6492)
}
