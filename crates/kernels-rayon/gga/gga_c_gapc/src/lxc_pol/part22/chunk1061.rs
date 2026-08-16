//! GGA_C_GAPC lxc pol — lxc_pol part 22 (v4rho2sigma2_1) CSE chunk 1061/1426 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part22_v4rho2sigma2_1_chunk1061(t11928: f64, t11931: f64, t11935: f64, t11939: f64, t11942: f64, t11946: f64, t11949: f64, t11951: f64, t11955: f64, t11958: f64, t11972: f64, t11981: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t12255 = 0.48340581405567281269e-8_f64 * t11928;
    let t12256 = 0.67528199161846004232e-6_f64 * t11931;
    let t12257 = 0.6746961805555555556e-5_f64 * t11935;
    let t12258 = 0.68714848362636882201e-6_f64 * t11939;
    let t12259 = 0.11254699860307667372e-7_f64 * t11942;
    let t12260 = 0.22098551499687900009e-8_f64 * t11946;
    let t12261 = 0.33147827249531850013e-7_f64 * t11949;
    let t12262 = 0.66295654499063700026e-7_f64 * t11951;
    let t12263 = 0.10120442708333333334e-3_f64 * t11955;
    let t12264 = 0.20240885416666666668e-4_f64 * t11958;
    let t12267 = 0.47342907336462418837e-4_f64 * t11972;
    let t12269 = 0.35848176214430067276e-9_f64 * t11981;
    (t12255, t12256, t12257, t12258, t12259, t12260, t12261, t12262, t12263, t12264, t12267, t12269)
}
