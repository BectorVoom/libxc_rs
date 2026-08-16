//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1283/1397 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk1283(t491: f64, t7321: f64, t1394: f64, t7924: f64, t1889: f64, t28814: f64, t95024: f64, t101853: f64, t23106: f64, t27556: f64, t27567: f64, t27583: f64, t29510: f64, t7986: f64, t99052: f64, t99058: f64, t99060: f64, t99065: f64, t99069: f64, t99082: f64, t99403: f64) -> (f64, f64, f64) {
    let t101978 = t7321 * t491;
    let t101980 = t1394 * t101978 * t7924;
    let t101985 = t95024 * t1889 * t28814;
    let t101991 = -t99052 + 0.46377350260416666667e-4_f64 * t27556 * t29510 - t99058 - 0.61890573922526041666e-5_f64 * t99060 - t99065 + 0.30918233506944444445e-4_f64 * t99069 - t99082 + 0.11607361111111111111e-2_f64 * t101980 - 0.18534722222222222222e-2_f64 * t101853 * t7986 - 0.30918233506944444445e-4_f64 * t27567 * t101985 + 0.30891203703703703704e-3_f64 * t27583 * t99403 * t23106;
    (t101980, t101985, t101991)
}
