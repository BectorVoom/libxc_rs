//! GGA_C_GAPC lxc pol — lxc_pol part 32 (v4rho2sigma2_11) CSE chunk 849/1311 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part32_v4rho2sigma2_11_chunk849(t9843: f64, t9846: f64, t7259: f64, t9067: f64, t8142: f64, t1084: f64, t9282: f64, t3415: f64, t9816: f64, t9818: f64, t9820: f64, t9822: f64, t9824: f64, t9828: f64, t9830: f64, t9833: f64, t9836: f64, t9839: f64) -> f64 {
    let t9847 = t9843 * t9846;
    let t9849 = t7259 * t9067;
    let t9850 = t9849 * t8142;
    let t9852 = t1084 * t9282;
    let t9853 = t9852 * t3415;
    let t9855 = -0.57970906942607043472e-5_f64 * t9816 + 0.28985453471303521736e-5_f64 * t9818 - 0.12163329537032409896e-2_f64 * t9820 - 0.6487109086417285278e-2_f64 * t9822 - 0.6487109086417285278e-2_f64 * t9824 + 0.14492726735651760868e-5_f64 * t9828 - 0.77294542590142724635e-6_f64 * t9830 + 0.1374296967252737644e-5_f64 * t9833 + 0.17376185052903442709e-3_f64 * t9836 + 0.25745714186718600948e-5_f64 * t9839 - 0.22919880997092966959e-8_f64 * t9847 - 0.21135226489492151266e-6_f64 * t9850 - 0.12380169846338434109e-5_f64 * t9853;
    t9855
}
