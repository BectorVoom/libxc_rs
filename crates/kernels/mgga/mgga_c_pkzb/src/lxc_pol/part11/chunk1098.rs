//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1098/1340 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1098<F: Float>(t20199: F, t2590: F, t2595: F, t17053: F, t2602: F, t2587: F, t5264: F, t2655: F, t1730: F, t2648: F, t16324: F, t177: F) -> (F, F, F, F, F, F) {
    let t20201 = t2590 * t20199 * t2595;
    let t20202 = F::cast_from(0.34013387707001991332e-1_f64) * t20201;
    let t20205 = t17053 * t2602;
    let t20221 = t5264 * t2587;
    let t20222 = F::cast_from(35.0_f64) / F::cast_from(72.0_f64) * t20221;
    let t20242 = t17053 * t2655;
    let t20261 = t1730 * t20199 * t2648;
    let t20262 = F::cast_from(0.17006693853500995666e-1_f64) * t20261;
    let t20267 = t16324 * t177;
    (t20202, t20205, t20222, t20242, t20262, t20267)
}
