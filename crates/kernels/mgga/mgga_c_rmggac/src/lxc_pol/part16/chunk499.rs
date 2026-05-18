//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 499/1158 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk499<F: Float>(t1941: F, t321: F, t333: F, t1743: F, t338: F, t352: F, t4697: F, t4705: F, t4997: F, t4998: F, t108: F, t1915: F) -> (F, F, F, F, F) {
    let t6332 = t1941 * t321;
    let t6335 = t1941 * t333;
    let t6338 = t338 * t1743;
    let t6339 = t6338 * t352;
    let t6344 = -t4697 - t4997 + t4998 + t4705;
    let t6349 = t1915 * t108;
    (t6332, t6335, t6339, t6344, t6349)
}
