//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 1327/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk1327<F: Float>(t1882: F, t31691: F, t31942: F, t31748: F, t8392: F, t10688: F, t112705: F, t113656: F, t113778: F, t11593: F, t125702: F, t15294: F, t18514: F, t1901: F, t19456: F, t19461: F, t19466: F, t24886: F, t24890: F, t2843: F, t296: F, t31814: F, t31857: F, t4151: F, t44528: F, t446: F, t5309: F, t6260: F, t684: F, t840: F, t98940: F, t98942: F) -> (F,) {
    let t126316 = t1882 * t31691;
    let t126320 = t1882 * t31942;
    let t126322 = t8392 * t31748;
    let t126328 = 8.0 / 81.0 * t113778 + 2.0 / 3.0 * t446 * t296 * t125702 - 2.0 / 3.0 * t446 * t840 * t10688 * t31814 - 2.0 / 3.0 * t446 * t840 * t2843 * t6260 * t5309 - 2.0 / 9.0 * t1901 * t24890 * t19461 - 2.0 / 9.0 * t1901 * t24886 * t19466 - 4.0 / 9.0 * t11593 * t24886 * t19456 + 2.0 / 9.0 * t1901 * t113656 * t4151 + 2.0 / 9.0 * t1901 * t44528 * t31857 * t684 + 2.0 / 9.0 * t126316 - 4.0 / 27.0 * t98940 - 4.0 / 27.0 * t98942 + 2.0 / 27.0 * t126320 + 2.0 / 27.0 * t126322 - 4.0 / 9.0 * t1901 * t15294 * t112705 * t18514;
    (t126328,)
}
