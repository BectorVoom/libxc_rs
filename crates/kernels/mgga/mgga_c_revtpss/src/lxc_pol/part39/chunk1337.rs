//! MGGA_C_REVTPSS lxc pol — lxc_pol part 39 (v4rho3tau_2) CSE chunk 1337/1348 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1337<F: Float>(t31027: F, t31277: F, t31032: F, t31284: F, t116912: F, t31261: F, t10208: F, t69: F, t96: F, t100: F, t1513: F, t2339: F, t31268: F, t101460: F, t101463: F, t10199: F, t116942: F, t1504: F, t2174: F, t2256: F, t2366: F, t31035: F, t31043: F, t31058: F, t31283: F, t4269: F, t8258: F, t8259: F, t8267: F, t8268: F) -> (F,) {
    let t117482 = 20.0 / 9.0 * t31027 * t31277;
    let t117484 = 20.0 / 27.0 * t31032 * t31284;
    let t117497 = 4.0 * t116912 * t31261;
    let t117499 = t69 * t10208 * t96;
    let t117500 = t100 * t1513;
    let t117505 = t69 * t2339 * t96;
    let t117510 = 20.0 / 9.0 * t31027 * t31268;
    let t117517 = -5.0 / 24.0 * t10199 * t2174 * t100 - t117482 + t117484 + 5.0 / 12.0 * t8258 * t8268 * t1504 * t2366 + 25.0 / 54.0 * t8267 * t116942 * t31283 - 5.0 / 36.0 * t8267 * t31058 * t1504 * t2256 + t117497 - 5.0 / 2.0 * t117499 * t117500 * t31043 + 5.0 / 9.0 * t117505 * t4269 * t31043 - t117510 - 3.0 / 2.0 * t31035 * t8259 * t101460 - 3.0 / 4.0 * t31035 * t8259 * t101463;
    (t117517,)
}
