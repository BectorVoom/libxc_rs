//! GGA_C_GAPC lxc pol — lxc_pol part 24 (v4rho2sigma2_3) CSE chunk 1321/1327 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part24_v4rho2sigma2_3_chunk1321<F: Float>(t35269: F, t35272: F, t35275: F, t35277: F, t35280: F, t35283: F, t35285: F, t35287: F, t35289: F, t35293: F, t35298: F, t35302: F, t35304: F, t35307: F, t35309: F, t35312: F, t35316: F, t35319: F, t35323: F, t35325: F, t35328: F, t35330: F) -> (F, F) {
    let t38472 = F::cast_from(0.16882592796244404291e-6_f64) * t35269 + F::cast_from(0.16009199995585360443e-6_f64) * t35272 + F::cast_from(0.16038463156432184077e-5_f64) * t35275 - F::cast_from(0.41034079713094038208e-8_f64) * t35277 - F::cast_from(0.67632724766374884054e-5_f64) * t35280 + F::cast_from(0.32076926312864368154e-5_f64) * t35283 + F::cast_from(0.25323889194366606437e-6_f64) * t35285 - F::cast_from(0.77347418024084520655e-7_f64) * t35287 - F::cast_from(0.10816602672322591148e-5_f64) * t35289 - F::cast_from(0.3787432586916993507e-3_f64) * t35293 + F::cast_from(0.3787432586916993507e-3_f64) * t35298;
    let t38485 = F::cast_from(0.43284943850479925794e-3_f64) * t35302 - F::cast_from(0.1376658521931966146e-6_f64) * t35304 - F::cast_from(0.33735894097222222226e-5_f64) * t35307 - F::cast_from(0.16193229166666666668e-3_f64) * t35309 + F::cast_from(0.97834092881944444454e-4_f64) * t35312 - F::cast_from(0.77294542590142724635e-7_f64) * t35316 + F::cast_from(0.29687586805555555558e-3_f64) * t35319 + F::cast_from(0.98396357783564814826e-6_f64) * t35323 + F::cast_from(0.12310223913928211462e-7_f64) * t35325 + F::cast_from(0.35904819748957283432e-8_f64) * t35328 + F::cast_from(0.43440462632258606772e-4_f64) * t35330;
    (t38472, t38485)
}
