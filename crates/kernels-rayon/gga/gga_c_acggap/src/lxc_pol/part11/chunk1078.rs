//! GGA_C_ACGGAP lxc pol — lxc_pol part 11 (v4rho3sigma_3) CSE chunk 1078/1213 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part11_v4rho3sigma_3_chunk1078(t30721: f64, t30725: f64, t30729: f64, t34743: f64, t34746: f64, t34747: f64, t34749: f64, t34751: f64, t34754: f64, t34757: f64, t34762: f64, t34767: f64, t34769: f64, t34771: f64, t34775: f64, t34779: f64, t34783: f64, t34788: f64) -> f64 {
    let t34790 = -t34743 - 0.18868855373762491241e-2_f64 * t30721 - t34746 - 0.68598428988911579156e-2_f64 * t34747 - 0.34299214494455789578e-2_f64 * t34749 + 0.17149607247227894789e-2_f64 * t34751 + t34754 + 0.64311027177104605458e-3_f64 * t34757 + 0.31448092289604152068e-2_f64 * t30725 + t30729 + 0.31448092289604152068e-3_f64 * t34762 - 0.41930789719472202758e-3_f64 * t34767 - 0.85748036236139473944e-3_f64 * t34769 - 0.41930789719472202758e-3_f64 * t34771 - 0.31448092289604152068e-3_f64 * t34775 - 0.62896184579208304136e-3_f64 * t34779 - 0.41930789719472202758e-3_f64 * t34783 - 0.31448092289604152068e-3_f64 * t34788;
    t34790
}
