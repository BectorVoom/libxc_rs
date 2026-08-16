//! MGGA_C_REVTPSS lxc pol — lxc_pol part 39 (v4rho3tau_2) CSE chunk 1158/1507 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1158(t1868: f64, t4010: f64, t1353: f64, t13767: f64, t2661: f64, t13756: f64, t13762: f64, t13763: f64, t13765: f64, t1410: f64, t9697: f64, t9705: f64, t9711: f64, t9712: f64, t9716: f64, t9725: f64, t9729: f64) -> (f64, f64) {
    let t13768 = t4010 * t1868;
    let t13769 = t13768 * t1353;
    let t13770 = t13767 * t13769;
    let t13772 = 0.28582678745379824648e-3_f64 * t2661 * t13770;
    let t13773 = 7.0_f64 / 144.0_f64 * t9697 - 0.14291339372689912324e-3_f64 * t9705 + t9711 - 0.60976381323476959249e-3_f64 * t9712 + 0.28582678745379824648e-4_f64 * t9716 + t9725 - t9729 - 0.85748036236139473944e-3_f64 * t1410 * t13756 - t13762 + 0.80031500487063509014e-2_f64 * t13763 + 0.54208002996571016773e-3_f64 * t13765 - t13772;
    (t13768, t13773)
}
