//! MGGA_C_RMGGAC lxc pol — lxc_pol part 14 (v4rho3sigma_5) CSE chunk 790/1089 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part14_v4rho3sigma_5_chunk790(t2084: f64, t27: f64, t7282: f64, t794: f64, t2160: f64, t638: f64, t7224: f64, t2184: f64, t465: f64, t7472: f64, t7478: f64, t118: f64, t1995: f64, t2001: f64, t498: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t36715 = t7282 * t27 * t2084 * t794;
    let t36718 = t638 * t2160 * t7224;
    let t36733 = t465 * t2184;
    let t36734 = t7472 * t36733;
    let t36735 = t36734 * t7478;
    let t36740 = t2001 * t118 * t1995 * t498;
    (t36715, t36718, t36733, t36734, t36735, t36740)
}
