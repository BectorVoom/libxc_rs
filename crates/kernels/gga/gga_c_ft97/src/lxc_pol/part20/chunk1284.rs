//! GGA_C_FT97 lxc pol — lxc_pol part 20 (v4rho3sigma_5) CSE chunk 1284/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part20_v4rho3sigma_5_chunk1284<F: Float>(t1882: F, t29356: F, t7059: F, t8232: F, t113540: F, t1476: F, t15126: F, t15175: F, t15527: F, t1901: F, t2409: F, t24886: F, t2739: F, t28496: F, t2862: F, t2874: F, t28843: F, t29071: F, t29259: F, t319: F, t446: F, t6273: F, t684: F, t7131: F, t835: F, t840: F, t882: F, t99848: F, t99850: F, t99867: F, t99885: F, t99887: F) -> (F,) {
    let t114869 = 4.0 / 9.0 * t1882 * t29356;
    let t114886 = t8232 * t7059;
    let t114894 = 2.0 / 9.0 * t99848 - 2.0 / 81.0 * t99850 + 4.0 / 3.0 * t446 * t2862 * t882 * t28496 + 4.0 / 3.0 * t446 * t2862 * t319 * t113540 + 2.0 / 9.0 * t1901 * t24886 * t15527 + 8.0 / 27.0 * t99867 - t114869 - 2.0 / 9.0 * t446 * t835 * t28843 * t684 - 2.0 / 9.0 * t1901 * t2874 * t29259 * t2409 - 4.0 * t1901 * t29071 * t6273 * t15175 - t446 * t840 * t7131 * t2739 / 3.0 - 4.0 / 27.0 * t114886 - t446 * t840 * t15126 * t1476 / 3.0 - 2.0 / 9.0 * t99885 - 2.0 / 9.0 * t99887;
    (t114894,)
}
