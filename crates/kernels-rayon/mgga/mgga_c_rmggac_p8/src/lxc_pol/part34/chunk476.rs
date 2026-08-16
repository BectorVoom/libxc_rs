//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 476/1097 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk476(t3076: f64, t352: f64, t2044: f64, t13839: f64, t3077: f64, t7501: f64, t211: f64, t384: f64, t1965: f64) -> (f64, f64, f64, f64, f64) {
    let t13840 = t3076 * t352;
    let t13841 = t2044 * t13840;
    let t13842 = t13839 * t13841;
    let t13844 = t7501 * t3077;
    let t13847 = t211 * t384;
    let t13848 = t1965 * t13847;
    (t13841, t13842, t13844, t13847, t13848)
}
