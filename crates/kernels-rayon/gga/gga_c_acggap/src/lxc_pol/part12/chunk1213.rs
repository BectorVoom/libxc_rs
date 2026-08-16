//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 1213/1250 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk1213(t36065: f64, t36081: f64, t36083: f64, t36085: f64, t36087: f64, t36089: f64, t31693: f64, t31697: f64, t31700: f64, t31702: f64, t31704: f64, t31708: f64, t32915: f64, t36063: f64, t36068: f64, t36077: f64, t36093: f64) -> f64 {
    let t37848 = 11.0_f64 / 144.0_f64 * t36065;
    let t37857 = 0.12579236915841660828e-2_f64 * t36081;
    let t37858 = 0.42874018118069736972e-3_f64 * t36083;
    let t37859 = 0.21437009059034868486e-2_f64 * t36085;
    let t37860 = 0.85748036236139473944e-3_f64 * t36087;
    let t37861 = 0.42874018118069736972e-3_f64 * t36089;
    let t37863 = t36063 / 24.0_f64 - t37848 + t36068 / 32.0_f64 + 0.28582678745379824648e-2_f64 * t31693 + 0.10718504529517434243e-2_f64 * t31697 - 0.57165357490759649296e-3_f64 * t31700 + 0.62896184579208304138e-3_f64 * t31702 + 0.83861579438944405516e-3_f64 * t31704 + 0.11433071498151929859e-2_f64 * t31708 + 0.37737710747524982482e-2_f64 * t36077 + t37857 - t32915 + t37858 + t37859 + t37860 - t37861 - 0.94344276868812456207e-3_f64 * t36093;
    t37863
}
