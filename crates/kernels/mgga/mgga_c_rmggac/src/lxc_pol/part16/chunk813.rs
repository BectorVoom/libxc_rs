//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 813/1158 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk813<F: Float>(t2186: F, t8582: F, t7288: F, t8659: F, t2286: F, t7921: F, t7244: F, t9171: F, t1540: F, t2144: F, t36734: F, t8443: F) -> (F, F, F, F, F, F) {
    let t39873 = t2186 * t8582;
    let t39899 = t8659 * t7288;
    let t39901 = t7921 * t2286;
    let t39926 = t7244 * t9171;
    let t39953 = t1540 * t2144;
    let t39970 = t36734 * t8443;
    (t39873, t39899, t39901, t39926, t39953, t39970)
}
