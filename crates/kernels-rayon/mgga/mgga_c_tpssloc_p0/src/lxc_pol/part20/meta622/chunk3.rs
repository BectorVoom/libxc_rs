//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2241/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2241(t52: f64, t10913: f64, t12606: f64, t12961: f64, t1431: f64, t2244: f64, t2250: f64, t4012: f64, t4015: f64, t4111: f64, t45872: f64, t607: f64, t771: f64, t78: f64, t9258: f64, t9288: f64, zeta_threshold: f64) -> f64 {
    let t150 = t52 <= zeta_threshold;
    let t46424 = piecewise3(t150, 0.0_f64, -56.0_f64 / 81.0_f64 * t4012 * t9288 - 8.0_f64 / 9.0_f64 * t4015 * t2244 - 8.0_f64 / 9.0_f64 * t1431 * t10913 - 2.0_f64 / 3.0_f64 * t78 * t12606 * t607 - 2.0_f64 / 3.0_f64 * t12961 * t2250 - 2.0_f64 / 9.0_f64 * t4111 * t9258 - 2.0_f64 / 3.0_f64 * t771 * t45872);
    t46424
}
