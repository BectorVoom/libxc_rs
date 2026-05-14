//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 1103/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk1103<F: Float>(t22541: F, t22543: F, t22572: F, t22522: F, t22524: F, t70: F, t93252: F, t22517: F, t22513: F, t1317: F, t23024: F, t376: F, t1642: F, t1800: F, t378: F, t8270: F) -> (F, F, F, F, F, F, F, F, F) {
    let t93319 = t22541 * t22572 * t22543;
    let t93322 = t22522 * t22572 * t22524;
    let t93324 = t93252 * t70;
    let t93325 = t93324 * t22517;
    let t93326 = t22513 * t93325;
    let t93349 = t1317 * t376 * t23024;
    let t93350 = t93349 / 6.0;
    let t93351 = t1642 * t1800;
    let t93355 = t378 * t8270;
    (t93319, t93322, t93324, t93325, t93326, t93349, t93350, t93351, t93355)
}
