//! MGGA_C_REVTPSS lxc pol — lxc_pol part 39 (v4rho3tau_2) CSE chunk 1162/1507 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1162(t5701: f64, t9962: f64, t13778: f64, t13779: f64, t13781: f64, t13786: f64, t13793: f64, t13797: f64, t13798: f64, t13801: f64, t13804: f64, t13807: f64, t13810: f64, t3934: f64, t5671: f64, t9735: f64) -> f64 {
    let t13813 = 0.20007875121765877254e-2_f64 * t9962 * t5701;
    let t13814 = t13778 - 0.76220476654346199061e-4_f64 * t13779 - 0.22675591804667994221e-1_f64 * t13781 - 0.85748036236139473944e-2_f64 * t3934 * t13786 - t9735 - 0.34299214494455789578e-2_f64 * t5671 * t13793 + t13797 - 35.0_f64 / 216.0_f64 * t13798 + 0.10164000561857065645e-4_f64 * t13801 - 0.12862205435420921092e-2_f64 * t13804 * t13807 - 0.80031500487063509015e-2_f64 * t13810 + t13813;
    t13814
}
