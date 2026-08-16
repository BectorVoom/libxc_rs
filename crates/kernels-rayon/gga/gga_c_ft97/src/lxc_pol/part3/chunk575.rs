//! GGA_C_FT97 lxc pol — lxc_pol part 3 (v3rho3_2) CSE chunk 575/1032 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part3_v3rho3_2_chunk575(t120: f64, t4441: f64, t72: f64, t126: f64, t4466: f64, t1631: f64, t2014: f64, t2021: f64, t4680: f64, t4683: f64, t534: f64, t139: f64) -> (f64, f64, f64, f64, f64) {
    let t4686 = t4441 * t120;
    let t4687 = t72 * t4686;
    let t4690 = t4441 * t126;
    let t4693 = t4466 * t126;
    let t4698 = -0.11705142615505742e0_f64 * t4680 * t120 + 0.23410285231011484e0_f64 * t4683 * t120 - 0.26564305359272358183e-2_f64 * t2014 * t4687 + 0.319782988780431561e-1_f64 * t2021 * t4690 - 0.532971647967385935e-1_f64 * t534 * t4693 + 0.13977476158628290272e-1_f64 * t1631 * t4690;
    let t4699 = t139 * t4698;
    (t4687, t4690, t4693, t4698, t4699)
}
