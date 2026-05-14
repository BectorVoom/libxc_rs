//! MGGA_C_KCISK lxc pol — lxc_pol part 23 (v4rho3sigma_3) CSE chunk 706/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part23_v4rho3sigma_3_chunk706<F: Float>(t2250: F, t4265: F, t139: F, t201: F, t41: F, t3529: F, t451: F, t5671: F, t1471: F, t2059: F, t4277: F, t1337: F, t5676: F, t1472: F, t220: F, t140: F, t2253: F, t299: F) -> (F, F, F, F, F, F, F, F, F) {
    let t6275 = t4265 * t2250;
    let t6278 = t139 * t201 * t41;
    let t6279 = t3529 * t451;
    let t6280 = t6279 * t5671;
    let t6284 = t1471 * t4277 * t2059;
    let t6287 = t1337 * t451;
    let t6288 = t6287 * t5676;
    let t6292 = t1471 * t1472 * t220;
    let t6296 = t140 * t299 * t2253;
    (t6275, t6278, t6279, t6280, t6284, t6287, t6288, t6292, t6296)
}
