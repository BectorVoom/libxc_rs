//! GGA_C_FT97 lxc pol — lxc_pol part 20 (v4rho3sigma_5) CSE chunk 1255/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part20_v4rho3sigma_5_chunk1255<F: Float>(t28719: F, t312: F, t29094: F, t8392: F, t29261: F, t29216: F, t10447: F, t10703: F, t112725: F, t113062: F, t11593: F, t14116: F, t15200: F, t15254: F, t15294: F, t15299: F, t15312: F, t15369: F, t15391: F, t15485: F, t1901: F, t2347: F, t2360: F, t24873: F, t24890: F, t24898: F, t2874: F, t2883: F, t29071: F, t29083: F, t29202: F, t29369: F, t3886: F, t4176: F, t44528: F, t6386: F, t684: F, t98899: F) -> (F,) {
    let t113710 = t312 * t28719;
    let t113716 = 4.0 / 81.0 * t8392 * t29094;
    let t113722 = 2.0 / 27.0 * t8392 * t29261;
    let t113749 = 2.0 / 27.0 * t8392 * t29216;
    let t113764 = 2.0 / 9.0 * t1901 * t2874 * t113710 * t684 + t113716 + 2.0 * t1901 * t29071 * t24898 * t15485 - t113722 - 4.0 / 3.0 * t1901 * t15369 * t98899 * t4176 + 2.0 / 9.0 * t1901 * t10447 * t29083 + 8.0 / 9.0 * t11593 * t15254 * t29202 * t14116 + t1901 * t24890 * t15391 / 9.0 - 4.0 / 9.0 * t1901 * t15312 * t112725 * t2883 + 2.0 / 9.0 * t1901 * t44528 * t24873 * t15200 + 4.0 / 9.0 * t1901 * t15299 * t113062 + t113749 - 4.0 / 9.0 * t1901 * t15254 * t6386 * t2360 * t3886 + 4.0 / 27.0 * t1901 * t15294 * t6386 * t2347 * t3886 - 2.0 / 9.0 * t1901 * t10703 * t29369 * t684;
    (t113764,)
}
