//! GGA_C_FT97 lxc pol — lxc_pol part 20 (v4rho3sigma_5) CSE chunk 1208/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part20_v4rho3sigma_5_chunk1208<F: Float>(t10491: F, t1508: F, t1501: F, t9570: F, t9577: F, t10703: F, t112663: F, t112671: F, t112679: F, t13863: F, t14075: F, t14678: F, t15183: F, t15254: F, t15294: F, t15295: F, t15303: F, t1901: F, t2409: F, t24940: F, t29202: F, t29207: F, t29399: F, t44523: F, t56098: F, t56418: F, t56437: F, t6334: F, t684: F, t7105: F, t99238: F) -> (F,) {
    let t112680 = t10491 * t1508;
    let t112696 = t1501 * t9570;
    let t112705 = t1501 * t9577;
    let t112713 = -2.0 / 9.0 * t1901 * t10703 * t29399 * t684 + 4.0 / 27.0 * t1901 * t112663 * t15295 + 2.0 / 9.0 * t1901 * t44523 * t6334 * t15183 + 2.0 / 3.0 * t1901 * t56418 * t112671 - 2.0 / 9.0 * t1901 * t99238 * t15303 + t112679 - 4.0 / 9.0 * t1901 * t112680 * t14678 + 2.0 / 3.0 * t1901 * t15254 * t29207 * t13863 + 2.0 / 9.0 * t1901 * t10703 * t7105 * t2409 + 2.0 / 27.0 * t1901 * t15294 * t29207 * t14075 + 10.0 / 81.0 * t1901 * t56437 * t112696 * t13863 - 2.0 / 9.0 * t1901 * t15254 * t29202 * t14075 - 4.0 / 9.0 * t1901 * t15294 * t112705 * t13863 - 2.0 / 9.0 * t1901 * t56098 * t24940;
    (t112713,)
}
