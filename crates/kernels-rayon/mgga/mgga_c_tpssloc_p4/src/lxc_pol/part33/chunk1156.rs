//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 33 (v4rho3sigma_9) CSE chunk 1156/1415 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part33_v4rho3sigma_9_chunk1156(t1799: f64, t1824: f64, t550: f64, t1339: f64, t22827: f64, t22833: f64, t6396: f64, t22820: f64, t22826: f64, t22859: f64, t22864: f64, t22868: f64, t26272: f64, t26295: f64, t28085: f64, t28089: f64, t28091: f64, t28093: f64, t28095: f64, t28097: f64) -> (f64, f64, f64) {
    let t28099 = t1799 * t1824;
    let t28100 = t28099 * t550;
    let t28101 = t1339 * t28100;
    let t28102 = t22827 * t28101;
    let t28104 = t22833 * t6396;
    let t28106 = 0.40372756094140390854e-3_f64 * t26272 + t28085 / 768.0_f64 - t22820 + t22826 + 0.28260929265898273598e-2_f64 * t26295 + t28089 / 1536.0_f64 - t28091 / 1536.0_f64 + 5.0_f64 / 384.0_f64 * t28093 - t28095 / 384.0_f64 - t28097 / 192.0_f64 + 0.24223653656484234512e-2_f64 * t28102 + t22859 + t22864 + t22868 + t28104 / 192.0_f64;
    (t28100, t28101, t28106)
}
