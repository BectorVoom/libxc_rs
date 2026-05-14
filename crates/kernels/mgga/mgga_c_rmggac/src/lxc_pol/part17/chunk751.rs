//! MGGA_C_RMGGAC lxc pol — lxc_pol part 17 (v4rho3sigma_8) CSE chunk 751/964 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part17_v4rho3sigma_8_chunk751<F: Float>(t41297: F, t41296: F, t8761: F, t1624: F, t2084: F, t8764: F, t1627: F, t7599: F, t1632: F, t8750: F, t7603: F, t25607: F, t27: F, t3851: F, t39692: F, t3826: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t41298 = 0.12122071846331262991e0 * t41297;
    let t41299 = t8761 * t41296;
    let t41300 = 0.45158592333657918156e-2 * t41299;
    let t41301 = t2084 * t1624;
    let t41302 = t8764 * t41301;
    let t41303 = 0.36366215538993788972e-1 * t41302;
    let t41307 = t2084 * t1627;
    let t41308 = t7599 * t41307;
    let t41313 = t2084 * t1632;
    let t41314 = t7599 * t41313;
    let t41315 = 0.72732431077987577946e-1 * t41314;
    let t41319 = t8750 * t41301;
    let t41320 = 0.2419210303588817044e-2 * t41319;
    let t41323 = t7603 * t41307;
    let t41324 = 0.33868944250243438616e-2 * t41323;
    let t41327 = t7603 * t41313;
    let t41329 = t25607 * t27;
    let t41338 = t3851 * t39692;
    let t41340 = t3826 * t39692;
    (t41298, t41300, t41303, t41308, t41315, t41320, t41324, t41327, t41329, t41338, t41340)
}
