//! MGGA_C_RMGGAC lxc pol — lxc_pol part 14 (v4rho3sigma_5) CSE chunk 895/952 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part14_v4rho3sigma_5_chunk895<F: Float>(t41276: F, t8761: F, t1635: F, t2084: F, t8746: F, t1624: F, t8764: F, t5181: F, t649: F, t36119: F, t1627: F, t7599: F, t5226: F, t36107: F, t1632: F, t5223: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t41294 = t8761 * t41276;
    let t41296 = t2084 * t1635;
    let t41297 = t8746 * t41296;
    let t41298 = 0.12122071846331262991e0 * t41297;
    let t41299 = t8761 * t41296;
    let t41300 = 0.45158592333657918156e-2 * t41299;
    let t41301 = t2084 * t1624;
    let t41302 = t8764 * t41301;
    let t41303 = 0.36366215538993788972e-1 * t41302;
    let t41304 = t649 * t5181;
    let t41305 = t36119 * t41304;
    let t41307 = t2084 * t1627;
    let t41308 = t7599 * t41307;
    let t41310 = t649 * t5226;
    let t41311 = t36107 * t41310;
    let t41313 = t2084 * t1632;
    let t41314 = t7599 * t41313;
    let t41315 = 0.72732431077987577946e-1 * t41314;
    let t41316 = t649 * t5223;
    (t41294, t41298, t41300, t41301, t41303, t41304, t41305, t41307, t41308, t41310, t41311, t41313, t41315, t41316)
}
