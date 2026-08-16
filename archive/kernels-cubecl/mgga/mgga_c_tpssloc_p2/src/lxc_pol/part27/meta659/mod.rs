//! MGGA_C_TPSSLOC lxc pol kernel — _part27_v4rho3sigma_3 meta659 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2301;
use chunk1::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2302;
use chunk2::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2303;
use chunk3::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2304;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_meta659<F: Float>(t26447: F, t90607: F, t90787: F, t22751: F, t26397: F, t22892: F, t22893: F, t26396: F, t26384: F, t16018: F, t6637: F, t6888: F, t6968: F, t1332: F, t26401: F, t90747: F, t90750: F, t90752: F, t90757: F, t90760: F, t90763: F, t90770: F, t90774: F, t90778: F, t90782: F, t90785: F, t26388: F, t7733: F, t81186: F, t5318: F, t552: F, t1307: F, t1352: F, t22633: F, t6976: F, t90754: F, t5187: F, t562: F, t1799: F, t81129: F, t22881: F, t16049: F, t1992: F, t81027: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t90789, t90792, t90795, t90798, t90801) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2301::<F>(t26447, t90607, t90787, t22751, t26397, t22892, t22893, t26396, t26384, t16018, t6637, t6888, t6968);
        let t90803 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2302::<F>(t1332, t26401, t90747, t90750, t90752, t90757, t90760, t90763, t90770, t90774, t90778, t90782, t90785, t90789, t90792, t90795, t90798, t90801);
        let (t90806, t90807, t90812, t90816) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2303::<F>(t22892, t22893, t26388, t7733, t81186, t5318, t552, t1307, t6637, t6888, t1352, t22633, t6976, t90754);
        let (t90818, t90821, t90825, t90829, t90832) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2304::<F>(t5187, t562, t1352, t22633, t6976, t1799, t6637, t6888, t81129, t22881, t16049, t1992, t81027);
    (t90803, t90806, t90807, t90812, t90816, t90818, t90821, t90825, t90829, t90832)
}
