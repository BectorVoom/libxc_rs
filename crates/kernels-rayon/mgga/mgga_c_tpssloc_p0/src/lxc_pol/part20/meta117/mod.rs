//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta117 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk775;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk776;
use chunk2::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk777;
use chunk3::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk778;
use chunk4::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk779;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta117(t2862: f64, t932: f64, t2764: f64, t2822: f64, t2766: f64, t2773: f64, t2778: f64, t2782: f64, t2800: f64, t2808: f64, t2816: f64, t2818: f64, t2824: f64, t2828: f64, t2831: f64, t2834: f64, t922: f64, t302: f64, t310: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t2863, t2868, t2875, t2880) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk775(t2862, t932, t2764, t2822, t2766, t2773, t2778, t2782, t2800, t2808, t2816, t2818, t2824, t2828, t2831, t2834);
        let (t2881, t2884) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk776(t2880, t932, t922);
        let t2885 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk777(t2884);
        let t2886 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk778(t2885, t302);
        let (t2887, t2888) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk779(t310);
    (t2863, t2868, t2875, t2880, t2881, t2884, t2885, t2886, t2887, t2888)
}
