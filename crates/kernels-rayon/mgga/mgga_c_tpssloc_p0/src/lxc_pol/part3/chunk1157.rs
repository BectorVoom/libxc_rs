//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 3 (v3rho3_1) CSE chunk 1157/1255 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part3_v3rho3_1_chunk1157(t14781: f64, t11137: f64, t11139: f64, t11141: f64, t11143: f64, t14728: f64, t14809: f64, t14811: f64, t14814: f64, t14816: f64, t14818: f64, t14824: f64) -> (f64, f64) {
    let t15094 = 0.27785333333333333334e0_f64 * t14781;
    let t15115 = -0.3529725e1_f64 * t14809 - 0.17648625e1_f64 * t14811 + 0.6311625e0_f64 * t14814 + 0.31558125e0_f64 * t14816 + 0.46308888888888888889e-1_f64 * t14818 + 0.45908888888888888888e0_f64 * t11137 + 0.11477222222222222222e0_f64 * t11139 - 0.34431666666666666666e0_f64 * t11141 - 0.17215833333333333333e0_f64 * t11143 + 0.6311625e0_f64 * t14824 + 0.57386111111111111112e0_f64 * t14728;
    (t15094, t15115)
}
