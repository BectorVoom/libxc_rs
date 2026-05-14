//! GGA_C_GAPC lxc pol — lxc_pol part 22 (v4rho2sigma2_1) CSE chunk 934/1209 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part22_v4rho2sigma2_1_chunk934<F: Float>(t11928: F, t11931: F, t11935: F, t11939: F, t11942: F, t11946: F, t11949: F, t11951: F, t11955: F, t11958: F, t11972: F, t11981: F, t11984: F, t11988: F, t11992: F, t11995: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t12255 = 0.48340581405567281269e-8 * t11928;
    let t12256 = 0.67528199161846004232e-6 * t11931;
    let t12257 = 0.6746961805555555556e-5 * t11935;
    let t12258 = 0.68714848362636882201e-6 * t11939;
    let t12259 = 0.11254699860307667372e-7 * t11942;
    let t12260 = 0.22098551499687900009e-8 * t11946;
    let t12261 = 0.33147827249531850013e-7 * t11949;
    let t12262 = 0.66295654499063700026e-7 * t11951;
    let t12263 = 0.10120442708333333334e-3 * t11955;
    let t12264 = 0.20240885416666666668e-4 * t11958;
    let t12267 = 0.47342907336462418837e-4 * t11972;
    let t12269 = 0.35848176214430067276e-9 * t11981;
    let t12270 = 0.33147827249531850013e-7 * t11984;
    let t12271 = 0.34752370105806885418e-3 * t11988;
    let t12272 = 0.4637672555408563478e-4 * t11992;
    let t12273 = 0.4637672555408563478e-4 * t11995;
    (t12255, t12256, t12257, t12258, t12259, t12260, t12261, t12262, t12263, t12264, t12267, t12269, t12270, t12271, t12272, t12273)
}
