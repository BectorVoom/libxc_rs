//! MGGA_C_TPSSLOC lxc pol kernel — _part27_v4rho3sigma_3 meta132 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk759;
use chunk1::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk760;
use chunk2::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk761;
use chunk3::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk762;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_meta132(t2904: f64, t315: f64, t950: f64, t951: f64, t2764: f64, t2822: f64, t2766: f64, t2773: f64, t2778: f64, t2782: f64, t2800: f64, t2808: f64, t2816: f64, t2818: f64, t2824: f64, t2828: f64, t2831: f64, t2834: f64, t941: f64, t323: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t2905, t2906) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk759(t2904, t315, t950);
        let (t2907, t2912, t2919, t2924) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk760(t2906, t951, t2764, t2822, t2766, t2773, t2778, t2782, t2800, t2808, t2816, t2818, t2824, t2828, t2831, t2834);
        let (t2925, t2928, t2929) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk761(t2924, t951, t941);
        let (t2930, t2931, t2932) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk762(t2929, t315, t323);
    (t2905, t2906, t2907, t2912, t2919, t2924, t2925, t2928, t2929, t2930, t2931, t2932)
}
