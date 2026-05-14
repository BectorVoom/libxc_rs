//! MGGA_C_RMGGAC lxc pol — lxc_pol part 17 (v4rho3sigma_8) CSE chunk 914/964 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part17_v4rho3sigma_8_chunk914<F: Float>(t39300: F, t40246: F, t1971: F, t236: F, t35331: F, t6135: F, t6139: F, t7365: F, t2157: F, t6349: F, t1916: F, t880: F, t2141: F, t1743: F, t2084: F, t2139: F, t27: F) -> (F, F, F, F, F, F) {
    let t47225 = t39300 * t40246;
    let t47229 = t35331 * t1971 * t236 * t6135;
    let t47233 = t7365 * t1971 * t236 * t6139;
    let t47235 = t6349 * t2157;
    let t47237 = t1916 * t880;
    let t47238 = t47237 * t2141;
    let t47242 = t2139 * t27 * t2084 * t1743;
    (t47225, t47229, t47233, t47235, t47238, t47242)
}
