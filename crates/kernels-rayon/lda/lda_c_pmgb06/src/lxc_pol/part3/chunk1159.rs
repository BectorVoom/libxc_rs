//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1159/1239 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk1159(t10403: f64, t10416: f64, t1447: f64, t5451: f64, t5454: f64, t5458: f64, t5499: f64, t1920: f64, t3226: f64, t5464: f64, t5467: f64, t5471: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t13834 = 4.0_f64 / 45.0_f64 * t10403;
    let t13835 = 4.0_f64 / 45.0_f64 * t10416;
    let t13836 = t1447 * t5451;
    let t13837 = 4.0_f64 / 45.0_f64 * t13836;
    let t13838 = t1447 * t5454;
    let t13839 = 4.0_f64 / 9.0_f64 * t13838;
    let t13840 = t5499 * t5458;
    let t13841 = 4.0_f64 / 9.0_f64 * t13840;
    let t13842 = t3226 * t1920;
    let t13843 = 4.0_f64 / 27.0_f64 * t13842;
    let t13844 = t1447 * t5464;
    let t13845 = 4.0_f64 / 27.0_f64 * t13844;
    let t13846 = t1447 * t5467;
    let t13847 = 2.0_f64 / 27.0_f64 * t13846;
    let t13848 = t1447 * t5471;
    (t13834, t13835, t13837, t13839, t13841, t13843, t13845, t13847, t13848)
}
