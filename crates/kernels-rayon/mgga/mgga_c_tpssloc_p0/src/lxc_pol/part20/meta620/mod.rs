//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta620 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2233;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2234;
use chunk2::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2235;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta620(t40748: f64, t40760: f64, t40764: f64, t40766: f64, t46282: f64, t46284: f64, t46286: f64, t46287: f64, t46288: f64, t46292: f64, t46293: f64, t2379: f64, t868: f64, t4199: f64, t9722: f64, t12887: f64, t172: f64, t763: f64, t12858: f64, t2535: f64, t40794: f64, t40804: f64, t40806: f64, t12606: f64, t707: f64, t751: f64, t40808: f64, t2749: f64, t776: f64, t12915: f64, t2522: f64, t39549: f64, t40797: f64, t40799: f64, t40801: f64, t40803: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t46294, t46298) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2233(t40748, t40760, t40764, t40766, t46282, t46284, t46286, t46287, t46288, t46292, t46293, t2379, t868);
        let (t46303, t46309, t46311, t46313, t46314, t46315, t46317) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2234(t4199, t9722, t12887, t172, t763, t12858, t2535, t40794, t40804, t40806, t12606, t707, t751);
        let (t46318, t46319, t46324) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2235(t46317, t40808, t2749, t776, t12915, t2522, t39549, t40797, t40799, t40801, t40803, t46313, t46314, t46315);
    (t46294, t46298, t46303, t46309, t46311, t46313, t46314, t46315, t46318, t46319, t46324)
}
