//! MGGA_C_TPSSLOC lxc pol kernel — _part27_v4rho3sigma_3 meta607 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2079;
use chunk1::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2080;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_meta607<F: Float>(t1011: F, t3120: F, t23384: F, t23650: F, t10336: F, t1920: F, t1949: F, t23323: F, t6781: F, t2966: F, t6805: F, t135: F, t23631: F, t6688: F, t23637: F, t23620: F, t968: F, t23617: F, t6680: F, t10454: F, t6765: F, t10889: F, t3033: F, t6753: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t82754, t82789, t82799, t82806, t82809, t82822) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2079::<F>(t1011, t3120, t23384, t23650, t10336, t1920, t1949, t23323, t6781, t2966, t6805, t135, t23631, t6688);
        let (t82823, t82828, t82830, t82843, t82848) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2080::<F>(t23637, t82822, t1920, t23620, t968, t23617, t6680, t10454, t6765, t10889, t3033, t6753);
    (t82754, t82789, t82799, t82806, t82809, t82822, t82823, t82828, t82830, t82843, t82848)
}
