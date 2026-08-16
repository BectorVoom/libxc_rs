//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 1051/1278 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk1051(t34990: f64, t35039: f64, t35041: f64, t35051: f64, t35070: f64, t35072: f64, t35074: f64, t35088: f64, t35090: f64, t35092: f64, t35096: f64, t35113: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t37321 = 0.57165357490759649296e-3_f64 * t34990;
    let t37361 = 7.0_f64 / 36.0_f64 * t35039;
    let t37362 = 7.0_f64 / 36.0_f64 * t35041;
    let t37365 = 0.28582678745379824648e-3_f64 * t35051;
    let t37372 = 0.16809375e0_f64 * t35070;
    let t37373 = 0.16809375e0_f64 * t35072;
    let t37374 = 0.1120625e0_f64 * t35074;
    let t37379 = 0.42874018118069736972e-3_f64 * t35088;
    let t37380 = 0.11321313224257494745e-1_f64 * t35090;
    let t37381 = 0.37737710747524982482e-2_f64 * t35092;
    let t37382 = 0.42874018118069736972e-2_f64 * t35096;
    let t37386 = 0.18868855373762491241e-1_f64 * t35113;
    (t37321, t37361, t37362, t37365, t37372, t37373, t37374, t37379, t37380, t37381, t37382, t37386)
}
