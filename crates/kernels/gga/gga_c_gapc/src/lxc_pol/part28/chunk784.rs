//! GGA_C_GAPC lxc pol — lxc_pol part 28 (v4rho2sigma2_7) CSE chunk 784/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part28_v4rho2sigma2_7_chunk784<F: Float>(t9843: F, t9846: F, t7259: F, t9067: F, t8142: F, t1084: F, t9282: F, t3415: F, t9816: F, t9818: F, t9820: F, t9822: F, t9824: F, t9828: F, t9830: F, t9833: F, t9836: F, t9839: F) -> (F, F, F, F) {
    let t9847 = t9843 * t9846;
    let t9849 = t7259 * t9067;
    let t9850 = t9849 * t8142;
    let t9852 = t1084 * t9282;
    let t9853 = t9852 * t3415;
    let t9855 = -0.57970906942607043472e-5 * t9816 + 0.28985453471303521736e-5 * t9818 - 0.12163329537032409896e-2 * t9820 - 0.6487109086417285278e-2 * t9822 - 0.6487109086417285278e-2 * t9824 + 0.14492726735651760868e-5 * t9828 - 0.77294542590142724635e-6 * t9830 + 0.1374296967252737644e-5 * t9833 + 0.17376185052903442709e-3 * t9836 + 0.25745714186718600948e-5 * t9839 - 0.22919880997092966959e-8 * t9847 - 0.21135226489492151266e-6 * t9850 - 0.12380169846338434109e-5 * t9853;
    (t9847, t9850, t9853, t9855)
}
