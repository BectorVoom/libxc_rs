//! MGGA_C_TPSSLOC lxc pol kernel — _part32_v4rho3sigma_8 meta119 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk701;
use chunk1::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk702;
use chunk2::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk703;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta119(t2887: f64, t2764: f64, t938: f64, t942: f64, t320: f64, t941: f64, t315: f64, t2822: f64, t323: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t2888, t2892, t2900, t2904) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk701(t2887, t2764, t938, t942, t320, t941);
        let (t2905, t2912, t2919, t2928, t2929) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk702(t2904, t315, t2764, t2822, t941);
        let (t2930, t2931, t2932) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk703(t2929, t315, t323);
    (t2888, t2892, t2900, t2904, t2905, t2912, t2919, t2928, t2929, t2930, t2931, t2932)
}
