//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 1179/1250 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk1179(t34745: f64, t34751: f64, t34753: f64, t30717: f64, t30721: f64, t30725: f64, t32561: f64, t34747: f64, t34749: f64, t34757: f64, t34762: f64, t34767: f64, t34769: f64, t34771: f64, t34775: f64, t34779: f64, t34783: f64, t34788: f64) -> f64 {
    let t37230 = 0.34299214494455789578e-2_f64 * t34745;
    let t37233 = 0.34299214494455789578e-2_f64 * t34751;
    let t37234 = 0.64025200389650807212e-1_f64 * t34753;
    let t37245 = -35.0_f64 / 54.0_f64 * t30717 - 0.37737710747524982482e-2_f64 * t30721 - t37230 - 0.13719685797782315831e-1_f64 * t34747 - 0.68598428988911579156e-2_f64 * t34749 + t37233 + t37234 + 0.12862205435420921092e-2_f64 * t34757 + 0.62896184579208304137e-2_f64 * t30725 + t32561 + 0.62896184579208304138e-3_f64 * t34762 - 0.83861579438944405518e-3_f64 * t34767 - 0.17149607247227894789e-2_f64 * t34769 - 0.83861579438944405518e-3_f64 * t34771 - 0.62896184579208304138e-3_f64 * t34775 - 0.12579236915841660828e-2_f64 * t34779 - 0.83861579438944405518e-3_f64 * t34783 - 0.62896184579208304138e-3_f64 * t34788;
    t37245
}
