//! MGGA_C_RMGGAC lxc pol — lxc_pol part 15 (v4rho3sigma_6) CSE chunk 836/1110 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part15_v4rho3sigma_6_chunk836<F: Float>(t41027: F, t851: F, t2118: F, t41032: F, t1635: F, t2084: F, t8746: F, t8761: F, t1624: F, t8764: F, t1627: F, t7599: F) -> (F, F, F, F, F, F, F, F) {
    let t41265 = t851 * t41027;
    let t41271 = t2118 * t41032;
    let t41296 = t2084 * t1635;
    let t41297 = t8746 * t41296;
    let t41298 = F::new(0.12122071846331262991e0) * t41297;
    let t41299 = t8761 * t41296;
    let t41300 = F::new(0.45158592333657918156e-2) * t41299;
    let t41301 = t2084 * t1624;
    let t41302 = t8764 * t41301;
    let t41303 = F::new(0.36366215538993788972e-1) * t41302;
    let t41307 = t2084 * t1627;
    let t41308 = t7599 * t41307;
    (t41265, t41271, t41298, t41300, t41301, t41303, t41307, t41308)
}
