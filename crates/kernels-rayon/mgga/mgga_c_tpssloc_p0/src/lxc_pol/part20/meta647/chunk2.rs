//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2377/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2377(t10756: f64, t1580: f64, t2930: f64, t10717: f64, t10720: f64, t10744: f64, t14271: f64, t42671: f64, t47798: f64, t47802: f64, t48725: f64, t48730: f64, t48732: f64, t48734: f64, t48736: f64, t48738: f64, t48741: f64, t48744: f64, t48771: f64, t48776: f64, t933: f64, t950: f64) -> f64 {
    let t48779 = t10756 * t1580;
    let t48783 = t2930 * t1580;
    let t48786 = 3.0_f64 * t48771 * t933 + 6.0_f64 * t14271 * t10744 - 0.57895126195293126243e3_f64 * t48776 * t10717 + 0.30762056574649219974e4_f64 * t48779 * t42671 * t950 + 0.10526802520742363173e2_f64 * t48783 * t10720 - t47798 - t47802 + t48725 + t48730 + t48732 + t48734 - t48736 - t48738 + t48741 + t48744;
    t48786
}
