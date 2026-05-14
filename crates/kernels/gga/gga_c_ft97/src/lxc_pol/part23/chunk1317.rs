//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 1317/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk1317<F: Float>(t31710: F, t8392: F, t25044: F, t5408: F, t19867: F, t6334: F, t19872: F, t31744: F, t10703: F, t112679: F, t11593: F, t15299: F, t1901: F, t19518: F, t29071: F, t29123: F, t29193: F, t29198: F, t29203: F, t29208: F, t29259: F, t31564: F, t31777: F, t4162: F, t56098: F, t57180: F, t684: F, t71524: F, t71528: F, t72163: F, t72397: F, t99098: F) -> (F, F, F, F) {
    let t125853 = t8392 * t31710;
    let t125858 = t25044 * t5408;
    let t125862 = t6334 * t19867;
    let t125866 = t6334 * t19872;
    let t125873 = t8392 * t31744;
    let t125898 = -t125853 / 27.0 - 4.0 / 9.0 * t1901 * t57180 * t31564 - 4.0 / 9.0 * t1901 * t15299 * t125858 - 4.0 / 9.0 * t1901 * t15299 * t125862 + 8.0 / 9.0 * t11593 * t15299 * t125866 + t112679 - 2.0 / 9.0 * t1901 * t99098 * t19518 - 2.0 / 81.0 * t125873 - 4.0 / 3.0 * t1901 * t72397 * t29123 - 2.0 / 9.0 * t1901 * t10703 * t31777 * t684 - 2.0 / 9.0 * t1901 * t56098 * t29193 - 4.0 / 9.0 * t1901 * t72163 * t29198 - 4.0 / 9.0 * t1901 * t71524 * t29203 + 4.0 / 27.0 * t1901 * t71528 * t29208 - 4.0 * t1901 * t29071 * t29259 * t4162;
    (t125858, t125862, t125866, t125898)
}
