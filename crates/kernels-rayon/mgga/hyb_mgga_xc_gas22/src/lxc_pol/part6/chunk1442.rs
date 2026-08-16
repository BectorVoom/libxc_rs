//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1442/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1442(t11335: f64, t9507: f64, t15686: f64, t9645: f64, t9656: f64, t1145: f64, t2889: f64, t4524: f64, t2893: f64, t11515: f64, t14626: f64, t26846: f64, t26850: f64, t31222: f64, t31229: f64, t31248: f64, t31330: f64, t7734: f64, t7780: f64, t9639: f64, t9663: f64, t9670: f64, t9678: f64, t9793: f64) -> (f64, f64, f64, f64, f64) {
    let t31347 = t11335 * t9507;
    let t31352 = t15686 * t9645;
    let t31355 = t15686 * t9656;
    let t31363 = t1145 * t4524 * t2889;
    let t31367 = t1145 * t4524 * t2893;
    let t31370 = -6400.0_f64 / 243.0_f64 * t9670 * t31229 + 1408.0_f64 / 243.0_f64 * t9678 * t31330 - 6400.0_f64 / 243.0_f64 * t9663 * t31229 + 704.0_f64 / 81.0_f64 * t9663 * t31222 - 704.0_f64 / 81.0_f64 * t9670 * t31347 - 704.0_f64 / 27.0_f64 * t9639 * t31347 - 2048.0_f64 / 243.0_f64 * t26846 * t31352 + 2048.0_f64 / 243.0_f64 * t26850 * t31355 + 40000.0_f64 / 81.0_f64 * t14626 * t31248 + 1600.0_f64 / 27.0_f64 * t9793 * t11515 + 126.0_f64 * t7734 * t31363 - 168.0_f64 * t7780 * t31367;
    (t31352, t31355, t31363, t31367, t31370)
}
