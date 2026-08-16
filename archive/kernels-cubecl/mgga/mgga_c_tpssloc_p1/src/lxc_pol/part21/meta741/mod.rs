//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta741 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2606;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2607;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta741<F: Float>(t11791: F, t5024: F, t11820: F, t5002: F, t11153: F, t4899: F, t3540: F, t4961: F, t11709: F, t15640: F, t1227: F, t13969: F, t15611: F, t15454: F, t4973: F, t49850: F, t11678: F, t11697: F, t15559: F, t15713: F, t3577: F, t45124: F, t1213: F, t1735: F, t248: F, t45017: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t52991, t52993, t52995, t52999, t53001, t53023) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2606::<F>(t11791, t5024, t11820, t5002, t11153, t4899, t3540, t4961, t11709, t15640, t1227, t13969, t15611);
        let (t53026, t53033, t53064, t53067, t53079) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2607::<F>(t1227, t13969, t15454, t4973, t49850, t11678, t11697, t15559, t15713, t3577, t45124, t1213, t1735, t248, t45017);
    (t52991, t52993, t52995, t52999, t53001, t53023, t53026, t53033, t53064, t53067, t53079)
}
