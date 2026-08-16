//! MGGA_C_TPSSLOC lxc pol kernel — _part31_v4rho3sigma_7 meta648 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1922;
use chunk1::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1923;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_meta648<F: Float>(t16828: F, t1888: F, t6646: F, t1484: F, t1519: F, t25038: F, t25248: F, t776: F, t232: F, t58262: F, t23110: F, t23185: F, t28422: F, t16817: F, t82018: F, t16825: F, t22996: F, t23168: F, t28346: F, t28338: F, t81591: F, t252: F, t5544: F, t22986: F, t829: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t98387, t98389, t98392, t98396, t98399) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1922::<F>(t16828, t1888, t6646, t1484, t1519, t25038, t25248, t776, t232, t58262, t23110, t23185, t28422);
        let (t98402, t98405, t98416, t98420, t98422, t98425) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1923::<F>(t16817, t1888, t82018, t16825, t22996, t23168, t28346, t28338, t81591, t252, t5544, t22986, t6646, t829);
    (t98387, t98389, t98392, t98396, t98399, t98402, t98405, t98416, t98420, t98422, t98425)
}
