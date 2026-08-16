//! MGGA_C_TPSSLOC lxc pol kernel — _part29_v4rho3sigma_5 meta577 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1994;
use chunk1::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1995;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_meta577<F: Float>(t22674: F, t22686: F, t80681: F, t22663: F, t6883: F, t225: F, t22624: F, t22622: F, t214: F, t3879: F, t22675: F, t22724: F, t22716: F, t6903: F, t22662: F, t6897: F, t22684: F, t6546: F, t22687: F, t131: F, t1365: F, t22648: F, t794: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t80683, t80689, t80699, t80704, t80707, t80711) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1994::<F>(t22674, t22686, t80681, t22663, t6883, t225, t22624, t22622, t214, t3879, t22675, t22724);
        let (t80722, t80725, t80727, t80728, t80730, t80738) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1995::<F>(t22716, t6903, t22662, t22674, t6897, t22684, t6546, t22687, t131, t1365, t22648, t794);
    (t80683, t80689, t80699, t80704, t80707, t80711, t80722, t80725, t80727, t80728, t80730, t80738)
}
