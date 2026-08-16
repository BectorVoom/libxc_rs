//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta380 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1838;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1839;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta380<F: Float>(t10224: F, t1592: F, t973: F, t2960: F, t4528: F, t1599: F, t698: F, t135: F, t4542: F, t13552: F, t13550: F, t13644: F, t10295: F, t10296: F, t10298: F, t10300: F, t10302: F, t13530: F, t13534: F, t13539: F, t13544: F, t13548: F, t13557: F, t13561: F, t13642: F, t13647: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t13895, t13896, t13907, t13908, t13909, t13913, t13915, t13921, t13922, t13923) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1838::<F>(t10224, t1592, t973, t2960, t4528, t1599, t698, t135, t4542, t13552, t13550, t13644);
        let t13931 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1839::<F>(t10295, t10296, t10298, t10300, t10302, t13530, t13534, t13539, t13544, t13548, t13557, t13561, t13642, t13647, t13921, t13922, t13923);
    (t13895, t13896, t13907, t13908, t13909, t13913, t13915, t13921, t13922, t13923, t13931)
}
