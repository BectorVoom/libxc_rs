//! GGA_C_GAPC lxc pol — lxc_pol part 25 (v4rho2sigma2_4) CSE chunk 1057/1444 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part25_v4rho2sigma2_4_chunk1057<F: Float>(t11919: F, t11928: F, t11931: F, t11935: F, t11939: F, t11942: F, t11946: F, t11949: F, t11951: F, t11955: F, t11958: F, t11972: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t12253 = F::cast_from(0.39291224566445086216e-8_f64) * t11919;
    let t12255 = F::cast_from(0.48340581405567281269e-8_f64) * t11928;
    let t12256 = F::cast_from(0.67528199161846004232e-6_f64) * t11931;
    let t12257 = F::cast_from(0.6746961805555555556e-5_f64) * t11935;
    let t12258 = F::cast_from(0.68714848362636882201e-6_f64) * t11939;
    let t12259 = F::cast_from(0.11254699860307667372e-7_f64) * t11942;
    let t12260 = F::cast_from(0.22098551499687900009e-8_f64) * t11946;
    let t12261 = F::cast_from(0.33147827249531850013e-7_f64) * t11949;
    let t12262 = F::cast_from(0.66295654499063700026e-7_f64) * t11951;
    let t12263 = F::cast_from(0.10120442708333333334e-3_f64) * t11955;
    let t12264 = F::cast_from(0.20240885416666666668e-4_f64) * t11958;
    let t12267 = F::cast_from(0.47342907336462418837e-4_f64) * t11972;
    (t12253, t12255, t12256, t12257, t12258, t12259, t12260, t12261, t12262, t12263, t12264, t12267)
}
