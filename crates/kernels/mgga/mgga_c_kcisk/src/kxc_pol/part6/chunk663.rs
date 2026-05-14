//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 663/957 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk663<F: Float>(t140: F, t3529: F, t3737: F, t11250: F, t461: F, t1337: F, t180: F, t479: F, t306: F, t425: F, t442: F, t3831: F, t458: F, t1202: F, t3721: F, t333: F, t3724: F) -> (F, F, F, F, F, F, F) {
    let t12841 = t140 * t3737 * t3529;
    let t12845 = 0.29201909629629629629e-3 * t11250 * t461;
    let t12847 = t180 * t479 * t1337;
    let t12848 = t306 * t425;
    let t12849 = t12848 * t442;
    let t12872 = t458 * t3831;
    let t12884 = 1.0 / t3721 / t1202;
    let t12888 = 1.0 / t3724 / t333;
    (t12841, t12845, t12847, t12849, t12872, t12884, t12888)
}
