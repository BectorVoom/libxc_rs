//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta121 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk796;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk797;
use chunk2::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk798;
use chunk3::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk799;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta121<F: Float>(t964: F, t969: F, t615: F, t972: F, t340: F, t697: F, t344: F, t221: F, t339: F, t135: F, t976: F, t979: F, t973: F, t986: F, t271: F, t883: F, t974: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t2958, t2960) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk796::<F>(t964, t969, t615, t972);
        let (t2965, t2967, t2969, t2970) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk797::<F>(t340, t697, t344, t221, t339, t135, t976);
        let (t2971, t2972, t2974, t2975, t2978) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk798::<F>(t2970, t979, t973, t135, t986, t271, t883);
        let t2979 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk799::<F>(t2978, t974);
    (t2958, t2960, t2965, t2967, t2969, t2970, t2971, t2972, t2974, t2975, t2978, t2979)
}
