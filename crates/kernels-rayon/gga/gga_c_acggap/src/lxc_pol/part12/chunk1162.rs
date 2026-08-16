//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 1162/1250 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk1162(t34027: f64, t34031: f64, t34033: f64, t34035: f64, t34037: f64, t34039: f64, t34043: f64, t34052: f64, t34056: f64, t34068: f64, t30260: f64, t30265: f64, t34029: f64, t34041: f64, t34048: f64, t34054: f64, t34059: f64, t34063: f64) -> f64 {
    let t36908 = 0.42874018118069736972e-2_f64 * t34027;
    let t36910 = 0.2264262644851498949e-1_f64 * t34031;
    let t36911 = 0.21437009059034868486e-3_f64 * t34033;
    let t36912 = 0.42874018118069736972e-3_f64 * t34035;
    let t36913 = 0.42874018118069736972e-3_f64 * t34037;
    let t36914 = 0.28582678745379824648e-3_f64 * t34039;
    let t36916 = 0.38110238327173099531e-2_f64 * t34043;
    let t36918 = 0.14291339372689912324e-2_f64 * t34052;
    let t36920 = 0.14291339372689912324e-2_f64 * t34056;
    let t36925 = 0.85748036236139473944e-3_f64 * t34068;
    let t36926 = t36908 + 0.25724410870841842184e-2_f64 * t34029 - t36910 - t36911 - t36912 - t36913 - t36914 - 0.17149607247227894789e-2_f64 * t34041 + t36916 - 0.10718504529517434243e-2_f64 * t34048 - t36918 - 0.26416397523267487738e-1_f64 * t34054 - t36920 - 0.27953859812981468505e-1_f64 * t30260 + 0.12579236915841660828e-2_f64 * t34059 - t34063 / 192.0_f64 - 0.83861579438944405516e-3_f64 * t30265 - t36925;
    t36926
}
