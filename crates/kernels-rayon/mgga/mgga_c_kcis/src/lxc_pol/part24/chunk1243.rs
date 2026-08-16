//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1243/1322 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk1243(t15534: f64, t4621: f64, t5281: f64, t100152: f64, t100235: f64, t100264: f64, t100268: f64, t100276: f64, t100280: f64, t26955: f64, t29117: f64, t7772: f64, t7791: f64, t92730: f64, t92749: f64, t93023: f64, t96868: f64) -> (f64, f64) {
    let t100284 = t15534 * t5281 * t4621;
    let t100289 = -0.61905925925925925925e-2_f64 * t100264 - 0.25794135802469135802e-3_f64 * t92730 - 0.11584201388888888889e-3_f64 * t100268 * t7791 - 0.92754700520833333334e-4_f64 * t7772 * t100235 + 0.13913205078125e-3_f64 * t7772 * t100152 - t96868 + 0.11584201388888888889e-3_f64 * t100276 - 0.10306077835648148148e-4_f64 * t92749 + 0.30918233506944444444e-4_f64 * t26955 * t100280 + 0.61836467013888888888e-4_f64 * t26955 * t100284 + 0.23168402777777777778e-3_f64 * t93023 * t29117;
    (t100284, t100289)
}
