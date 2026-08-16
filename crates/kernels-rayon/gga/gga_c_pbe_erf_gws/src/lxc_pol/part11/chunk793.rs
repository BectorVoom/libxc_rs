//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 793/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk793(t12501: f64, t1714: f64, t12505: f64, t657: f64, t12509: f64, t10519: f64, t10521: f64, t10581: f64, t10583: f64, t10585: f64, t12495: f64, t12515: f64, t12837: f64, t25: f64) -> (f64, f64, f64, f64) {
    let t12840 = t1714 * t12501;
    let t12843 = t657 * t12505;
    let t12846 = t657 * t12509;
    let t12854 = -0.39990740740740740742e-1_f64 * t12495 - 0.35991666666666666667e-1_f64 * t12515 + 0.13333333333333333333e-1_f64 * t25 * t12837 - 0.66666666666666666666e-2_f64 * t25 * t12840 - 0.39999999999999999999e-1_f64 * t25 * t12843 + 0.39999999999999999999e-1_f64 * t25 * t12846 - 0.26666666666666666667e-1_f64 * t10519 + 0.13333333333333333334e-1_f64 * t10521 + 0.35991666666666666666e-1_f64 * t10585 + 0.23994444444444444444e-1_f64 * t10581 - 0.71983333333333333333e-1_f64 * t10583;
    (t12840, t12843, t12846, t12854)
}
