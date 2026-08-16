//! MGGA_C_TPSSLOC lxc pol kernel — _part31_v4rho3sigma_7 meta616 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1863;
use chunk1::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1864;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_meta616<F: Float>(t5464: F, t666: F, t81446: F, t1453: F, t4067: F, t22473: F, t22470: F, t5488: F, t19529: F, t6530: F, t7684: F, t8944: F, t1390: F, t19631: F, t1845: F, t5356: F, t22674: F, t28191: F, t80681: F, t1985: F, t22666: F, t28232: F, t26331: F, t26333: F, t90566: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t96716, t96719, t96721, t96724, t96726, t96797) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1863::<F>(t5464, t666, t81446, t1453, t4067, t22473, t22470, t5488, t19529, t6530, t7684, t8944);
        let (t96824, t96830, t96848, t96851, t96854) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1864::<F>(t1390, t19631, t1845, t5356, t22674, t28191, t80681, t1985, t22666, t28232, t26331, t26333, t90566);
    (t96716, t96719, t96721, t96724, t96726, t96797, t96824, t96830, t96848, t96851, t96854)
}
