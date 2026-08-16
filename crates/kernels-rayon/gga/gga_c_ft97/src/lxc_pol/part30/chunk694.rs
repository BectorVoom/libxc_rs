//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 694/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk694(t4162: f64, t6273: f64, t29071: f64, t24898: f64, t4167: f64, t15369: f64, t7124: f64, t870: f64, t684: f64, t2881: f64, t24886: f64, t4261: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t29072 = t6273 * t4162;
    let t29073 = t29071 * t29072;
    let t29076 = t24898 * t4167;
    let t29077 = t15369 * t29076;
    let t29082 = t870 * t7124;
    let t29083 = t29082 * t684;
    let t29084 = t2881 * t29083;
    let t29087 = t24886 * t4261;
    (t29072, t29073, t29076, t29077, t29083, t29084, t29087)
}
