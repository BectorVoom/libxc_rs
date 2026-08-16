//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 60 (v4rho2sigma2_16) CSE chunk 491/1064 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part60_v4rho2sigma2_16_chunk491(t1088: f64, t5979: f64, t123: f64, t3237: f64, t4721: f64, t5973: f64, t5977: f64, t423: f64, t1671: f64, t4740: f64, t1670: f64, t1118: f64) -> (f64, f64, f64, f64, f64) {
    let t5980 = t1088 * t5979;
    let t5981 = t123 * t5980;
    let t5983 = t3237 - 0.11872222222222222222e-1_f64 * t4721 - 0.11872222222222222222e-1_f64 * t5973 + 0.35616666666666666666e-1_f64 * t5977 + 0.17808333333333333333e-1_f64 * t5981;
    let t5985 = 0.621814e-1_f64 * t5983 * t423;
    let t5987 = 2.0_f64 * t4740 * t1671;
    let t5988 = t1670 * t1670;
    let t5989 = t5988 * t1118;
    (t5981, t5985, t5987, t5988, t5989)
}
