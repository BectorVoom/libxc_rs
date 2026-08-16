//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 843/1278 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk843(t8233: f64, t8235: f64, t8240: f64, t8835: f64, t8841: f64, t8847: f64, t8851: f64, t8860: f64, t8862: f64, t8876: f64, t8879: f64, t8882: f64, t9682: f64, t9688: f64, t9692: f64, t9694: f64, t9696: f64, t9698: f64, t9702: f64, t9706: f64) -> f64 {
    let t9940 = -0.10718504529517434243e-2_f64 * t9682 - t8233 + 0.80031500487063509014e-2_f64 * t8835 + 0.34299214494455789578e-2_f64 * t8841 - 0.34299214494455789578e-2_f64 * t8847 + 0.42874018118069736972e-3_f64 * t8851 + 0.28582678745379824648e-3_f64 * t8860 + 0.21437009059034868486e-3_f64 * t9688 - 0.62896184579208304138e-3_f64 * t9692 - 0.34299214494455789578e-2_f64 * t9694 + 0.17149607247227894789e-2_f64 * t9696 - 0.17149607247227894789e-2_f64 * t9698 + 0.22921875e-1_f64 * t9702 + 0.1528125e-1_f64 * t9706 + 0.75475421495049964965e-2_f64 * t8862 - t8876 / 16.0_f64 - t8879 / 48.0_f64 + t8235 - t8240 - 0.1120625e0_f64 * t8882;
    t9940
}
