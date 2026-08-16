//! MGGA_C_TPSSLOC lxc pol kernel — _part31_v4rho3sigma_7 meta622 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1877;
use chunk1::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1878;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_meta622<F: Float>(t1825: F, t22633: F, t6976: F, t90818: F, t26421: F, t5287: F, t22751: F, t28149: F, t19740: F, t1992: F, t22897: F, t28139: F, t28159: F, t6897: F, t794: F, t19763: F, t19739: F, t3807: F, t28131: F, t81159: F, t552: F, t6434: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t97087, t97091, t97095, t97106, t97108) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1877::<F>(t1825, t22633, t6976, t90818, t26421, t5287, t22751, t28149, t19740, t1992, t22897, t28139);
        let (t97111, t97114, t97119, t97124, t97126) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1878::<F>(t28159, t6897, t794, t19763, t1992, t6976, t19739, t22633, t3807, t28131, t81159, t552, t6434);
    (t97087, t97091, t97095, t97106, t97108, t97111, t97114, t97119, t97124, t97126)
}
