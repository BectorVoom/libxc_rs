//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 1990/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1990(t13999: f64, t5677: f64, t13967: f64, t13977: f64, t13981: f64, t13987: f64, t13988: f64, t13991: f64, t13995: f64, t3934: f64, t5671: f64, t9847: f64, t9896: f64, t9906: f64, t9910: f64, t9919: f64) -> (f64, f64) {
    let t14001 = 0.40015750243531754508e-2_f64 * t13999 * t5677;
    let t14002 = 0.85748036236139473944e-3_f64 * t3934 * t13967 + 0.50820002809285328224e-5_f64 * t9847 + 0.10003937560882938627e-2_f64 * t9896 - 0.12705000702321332056e-4_f64 * t9906 - 0.11337795902333997111e-1_f64 * t9910 - 0.20007875121765877254e-2_f64 * t9919 + 0.17149607247227894789e-2_f64 * t3934 * t13977 + 0.85748036236139473944e-3_f64 * t3934 * t13981 + t13987 - 0.80031500487063509014e-2_f64 * t13988 + 0.85748036236139473944e-3_f64 * t5671 * t13991 - 0.42874018118069736972e-2_f64 * t3934 * t13995 - t14001;
    (t14001, t14002)
}
