//! MGGA_C_REVTPSS lxc pol — lxc_pol part 56 (v4rho2sigma2_11) CSE chunk 1155/1203 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part56_v4rho2sigma2_11_chunk1155(t5795: f64, t8614: f64, t1459: f64, t34007: f64, t1916: f64, t32366: f64, t32855: f64, t4248: f64, t27123: f64, t8749: f64, t27126: f64, t32866: f64, t7732: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t127495 = 3.0_f64 * t5795 * t8614;
    let t127503 = 12.0_f64 * t1459 * t34007;
    let t127507 = 6.0_f64 * t1916 * t32366;
    let t129251 = t4248 * t32855;
    let t129253 = t27123 * t8749;
    let t129255 = t27126 * t8749;
    let t129257 = t7732 * t32866;
    (t127495, t127503, t127507, t129251, t129253, t129255, t129257)
}
