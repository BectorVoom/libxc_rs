//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3901/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3901(t14230: f64, t2782: f64, t48083: f64, t4086: f64, t543: f64, t74922: f64, t10073: f64, t22365: f64, t14141: f64, t14143: f64, t5658: f64, t676: f64) -> (f64, f64, f64, f64) {
    let t74935 = t2782 * t48083 * t14230;
    let t74943 = t2782 * t4086 * t74922 * t543;
    let t74945 = t10073 * t22365;
    let t74949 = t14141 * t14143 * t676 * t5658;
    (t74935, t74943, t74945, t74949)
}
