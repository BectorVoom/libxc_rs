//! MGGA_C_R2SCAN lxc pol — lxc_pol part 18 (v4rho3sigma_8) CSE chunk 268/1112 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part18_v4rho3sigma_8_chunk268<F: Float>(t333: F, t839: F, t335: F, t337: F, t339: F, t341: F, t349: F) -> (F, F, F, F, F, F, F) {
    let t841 = t333 * t839;
    let t843 = t335 * t839;
    let t845 = t337 * t839;
    let t847 = t339 * t839;
    let t849 = t341 * t839;
    let t854 = t349 * t349;
    let t855 = 1.0 / t854;
    (t841, t843, t845, t847, t849, t854, t855)
}
