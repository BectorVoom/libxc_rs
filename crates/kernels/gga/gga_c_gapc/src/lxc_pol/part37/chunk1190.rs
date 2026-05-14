//! GGA_C_GAPC lxc pol — lxc_pol part 37 (v4rho2sigma2_16) CSE chunk 1190/1225 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part37_v4rho2sigma2_16_chunk1190<F: Float>(t35231: F, t35234: F, t35240: F, t35243: F, t35246: F, t35249: F, t35252: F, t35254: F, t35257: F, t35259: F, t35263: F, t35269: F, t35272: F, t35275: F, t35277: F, t35280: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t37244 = 0.11372686522837130914e-5 * t35231;
    let t37245 = 0.10567613244746075633e-6 * t35234;
    let t37249 = 0.40483072916666666669e-4 * t35240;
    let t37250 = 0.2698871527777777778e-4 * t35243;
    let t37251 = 0.2698871527777777778e-4 * t35246;
    let t37252 = 0.17149079499421296297e-4 * t35249;
    let t37253 = 0.2748593934505475288e-6 * t35252;
    let t37254 = 0.36652500116630512966e-6 * t35254;
    let t37255 = 0.41030519691600762993e-3 * t35257;
    let t37256 = 0.94685814672924837674e-4 * t35259;
    let t37257 = 0.1500544456199363426e-4 * t35263;
    let t37260 = 0.84412963981222021456e-7 * t35269;
    let t37261 = 0.80045999977926802214e-7 * t35272;
    let t37262 = 0.80192315782160920384e-6 * t35275;
    let t37263 = 0.20517039856547019104e-8 * t35277;
    let t37264 = 0.33816362383187442026e-5 * t35280;
    (t37244, t37245, t37249, t37250, t37251, t37252, t37253, t37254, t37255, t37256, t37257, t37260, t37261, t37262, t37263, t37264)
}
