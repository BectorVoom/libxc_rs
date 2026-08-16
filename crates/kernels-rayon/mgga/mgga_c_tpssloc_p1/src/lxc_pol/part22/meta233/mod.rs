//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta233 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1303;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1304;
use chunk2::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1305;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta233(t731: f64, t9751: f64, t746: f64, t9490: f64, t172: f64, t9489: f64, t9493: f64, t9720: f64, t2512: f64, t9711: f64, t9689: f64, t9692: f64, t9695: f64, t9698: f64, t9702: f64, t9704: f64, t9706: f64, t9709: f64, t702: f64, t683: f64, t9731: f64, t2405: f64, t2420: f64, t703: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t9752, t9755, t9758, t9759, t9762, t9763, t9766, t9777) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1303(t731, t9751, t746, t9490, t172, t9489, t9493, t9720, t2512, t9711, t9689, t9692, t9695, t9698, t9702, t9704, t9706, t9709);
        let (t9778, t9780) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1304(t702, t9777, t683);
        let (t9781, t9789) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1305(t731, t9731, t2405, t2420, t703);
    (t9752, t9755, t9758, t9759, t9762, t9763, t9766, t9777, t9778, t9780, t9781, t9789)
}
