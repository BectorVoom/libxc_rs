//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 993/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk993<F: Float>(t77398: F, t1550: F, t2069: F, t2471: F, t2074: F, t903: F, t75006: F, t71300: F, t75002: F, t75003: F, t75005: F, t77370: F, t77372: F, t77374: F, t77376: F, t77379: F, t77384: F, t77388: F, t77392: F, t77397: F) -> F {
    let t77399 = F::cast_from(0.42564599893297839398e-5_f64) * t77398;
    let t77401 = t1550 * t2471 * t2069;
    let t77402 = F::cast_from(0.2993560425465952141e-1_f64) * t77401;
    let t77404 = t903 * t2471 * t2074;
    let t77405 = F::cast_from(0.44903406381989282115e-1_f64) * t77404;
    let t77406 = F::cast_from(0.85129199786595678799e-5_f64) * t75006;
    let t77407 = t77370 + t77372 + t77374 - t75002 - t71300 - t77376 - t77379 - t75003 + t75005 + t77384 - t77388 + t77392 + t77397 - t77399 + t77402 - t77405 + t77406;
    t77407
}
