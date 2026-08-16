//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 2106/2259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2106(t105946: f64, t7060: f64, t105936: f64, t92843: f64, t29658: f64, t686: f64, t72: f64, t7058: f64, t7064: f64, t105934: f64, t105937: f64, t105939: f64, t27349: f64, t92858: f64, t93349: f64, t98803: f64, t98806: f64, t98811: f64, t98814: f64, t98817: f64, t99414: f64) -> f64 {
    let t105947 = t105946 * t7060;
    let t105949 = t92843 * t105936;
    let t105953 = t29658 * t72 * t686;
    let t105954 = t7058 * t105953;
    let t105956 = t7064 * t105953;
    let t105958 = -t98803 + t98806 + 0.14456046980341999104e-1_f64 * t105934 + t98811 - t98814 - t98817 - 0.51405703062096148813e-1_f64 * t105937 - 0.25702851531048074406e-1_f64 * t105939 + 0.52041769129231196772e1_f64 * t93349 * t99414 * t27349 + 0.72280234901709995518e-2_f64 * t105947 + 0.28912093960683998207e-1_f64 * t105949 - 0.73171657588172351096e-2_f64 * t92858 + 0.72280234901709995518e-2_f64 * t105954 - 0.12851425765524037203e-1_f64 * t105956;
    t105958
}
