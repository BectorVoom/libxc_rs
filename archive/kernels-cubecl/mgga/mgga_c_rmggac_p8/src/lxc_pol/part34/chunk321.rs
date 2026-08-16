//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 321/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk321<F: Float>(t3093: F, t3094: F, t3046: F, t851: F, t637: F, t854: F, t1322: F, t861: F, t194: F) -> (F, F, F, F, F, F) {
    let t3095 = t3093 * t3094;
    let t3097 = t851 * t3046;
    let t3100 = t854 * t3046 * t637;
    let t3102 = t861 * t1322;
    let t3103 = t3102 * t3094;
    let t3112 = F::cast_from(1.0_f64) / t194;
    (t3095, t3097, t3100, t3102, t3103, t3112)
}
