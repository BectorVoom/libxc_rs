//! MGGA_C_TPSSLOC lxc pol kernel — _part23_v4rho4_4 meta87 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk499;
use chunk1::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk500;
use chunk2::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk501;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_meta87(t2840: f64, t275: f64, t290: f64, t2764: f64, t307: f64, t922: f64, t302: f64, t2822: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t2841, t2842) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk499(t2840, t275);
        let (t2843, t2844) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk500(t290);
        let (t2848, t2859, t2860, t2861, t2868, t2875, t2884) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk501(t2764, t307, t922, t302, t2822);
    (t2841, t2842, t2843, t2844, t2848, t2859, t2860, t2861, t2868, t2875, t2884)
}
