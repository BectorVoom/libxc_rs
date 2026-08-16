//! MGGA_C_TPSSLOC lxc pol kernel — _part27_v4rho3sigma_3 meta659 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2301;
use chunk1::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2302;
use chunk2::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2303;
use chunk3::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2304;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_meta659(t26447: f64, t90607: f64, t90787: f64, t22751: f64, t26397: f64, t22892: f64, t22893: f64, t26396: f64, t26384: f64, t16018: f64, t6637: f64, t6888: f64, t6968: f64, t1332: f64, t26401: f64, t90747: f64, t90750: f64, t90752: f64, t90757: f64, t90760: f64, t90763: f64, t90770: f64, t90774: f64, t90778: f64, t90782: f64, t90785: f64, t26388: f64, t7733: f64, t81186: f64, t5318: f64, t552: f64, t1307: f64, t1352: f64, t22633: f64, t6976: f64, t90754: f64, t5187: f64, t562: f64, t1799: f64, t81129: f64, t22881: f64, t16049: f64, t1992: f64, t81027: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t90789, t90792, t90795, t90798, t90801) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2301(t26447, t90607, t90787, t22751, t26397, t22892, t22893, t26396, t26384, t16018, t6637, t6888, t6968);
        let t90803 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2302(t1332, t26401, t90747, t90750, t90752, t90757, t90760, t90763, t90770, t90774, t90778, t90782, t90785, t90789, t90792, t90795, t90798, t90801);
        let (t90806, t90807, t90812, t90816) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2303(t22892, t22893, t26388, t7733, t81186, t5318, t552, t1307, t6637, t6888, t1352, t22633, t6976, t90754);
        let (t90818, t90821, t90825, t90829, t90832) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2304(t5187, t562, t1352, t22633, t6976, t1799, t6637, t6888, t81129, t22881, t16049, t1992, t81027);
    (t90803, t90806, t90807, t90812, t90816, t90818, t90821, t90825, t90829, t90832)
}
