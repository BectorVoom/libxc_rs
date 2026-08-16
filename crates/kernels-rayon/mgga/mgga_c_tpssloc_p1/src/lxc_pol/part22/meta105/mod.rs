//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta105 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk714;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk715;
use chunk2::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk716;
use chunk3::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk717;
use chunk4::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk718;
use chunk5::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk719;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta105(t310: f64, t2764: f64, t938: f64, t942: f64, t320: f64, t941: f64, t315: f64, t2822: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t2887, t2888) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk714(t310);
        let (t2892, t2900, t2903, t2904) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk715(t2764, t938, t942, t320, t941);
        let t2905 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk716(t2904, t315);
        let (t2912, t2919, t2928) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk717(t2764, t2822, t941);
        let t2929 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk718(t2928);
        let t2930 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk719(t2929, t315);
    (t2887, t2888, t2892, t2900, t2903, t2904, t2905, t2912, t2919, t2928, t2929, t2930)
}
