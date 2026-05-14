//! GGA_C_FT97 lxc pol — lxc_pol part 20 (v4rho3sigma_5) CSE chunk 949/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part20_v4rho3sigma_5_chunk949<F: Float>(t4162: F, t6273: F, t29071: F, t24898: F, t4167: F, t15369: F, t7124: F, t870: F, t684: F, t2881: F, t24886: F, t4261: F, t4266: F, t1495: F, t2766: F, t4141: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t29072 = t6273 * t4162;
    let t29073 = t29071 * t29072;
    let t29076 = t24898 * t4167;
    let t29077 = t15369 * t29076;
    let t29082 = t870 * t7124;
    let t29083 = t29082 * t684;
    let t29084 = t2881 * t29083;
    let t29087 = t24886 * t4261;
    let t29090 = t24886 * t4266;
    let t29093 = t2766 * t1495;
    let t29094 = t29093 * t4141;
    (t29072, t29073, t29076, t29077, t29082, t29083, t29084, t29087, t29090, t29093, t29094)
}
