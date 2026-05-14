//! GGA_C_FT97 lxc pol — lxc_pol part 20 (v4rho3sigma_5) CSE chunk 1212/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part20_v4rho3sigma_5_chunk1212<F: Float>(t13863: F, t99322: F, t14116: F, t25140: F, t29098: F, t8392: F, t1882: F, t29182: F, t29060: F, t11593: F, t15229: F, t15290: F, t15377: F, t15518: F, t1901: F, t24890: F, t24899: F, t24908: F, t29071: F, t29093: F, t29137: F, t4162: F, t4266: F, t56127: F, t69996: F, t98738: F, t98746: F, t98751: F, t98753: F, t99034: F) -> (F, F, F) {
    let t112812 = t99322 * t13863;
    let t112816 = t25140 * t14116;
    let t112821 = 2.0 / 27.0 * t8392 * t29098;
    let t112831 = 2.0 / 9.0 * t1882 * t29182;
    let t112848 = 2.0 / 27.0 * t8392 * t29060;
    let t112849 = -4.0 / 9.0 * t1901 * t15290 * t112812 + 8.0 / 9.0 * t11593 * t15229 * t112816 - t112821 - 2.0 / 9.0 * t1901 * t24890 * t15518 + 4.0 / 9.0 * t98738 + 4.0 / 9.0 * t1901 * t99034 * t4266 + 2.0 / 9.0 * t98746 + t112831 - 4.0 * t1901 * t29071 * t24908 * t4162 - 4.0 / 3.0 * t1901 * t56127 * t29137 - 8.0 / 27.0 * t98751 - 8.0 / 27.0 * t98753 + 2.0 / 27.0 * t1901 * t29093 * t15377 - 4.0 / 3.0 * t1901 * t69996 * t24899 - t112848;
    (t112812, t112816, t112849)
}
