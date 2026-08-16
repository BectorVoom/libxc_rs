//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 25 (v4rho3sigma_1) CSE chunk 849/1226 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part25_v4rho3sigma_1_chunk849(t1040: f64, t3077: f64, t2775: f64, t283: f64, t61: f64, t10305: f64, t248: f64, t135: f64, t3142: f64, t973: f64, t3147: f64, t9258: f64, t998: f64) -> (f64, f64, f64, f64, f64) {
    let t10965 = t3077 * t1040;
    let t10969 = 1.0_f64 / t283 / t2775;
    let t10970 = t61 * t10969;
    let t10972 = t248 * t10970 * t10305;
    let t10981 = t135 * t3142;
    let t10982 = t973 * t10981;
    let t10984 = t135 * t3147;
    let t10985 = t973 * t10984;
    let t10987 = t998 * t9258;
    (t10965, t10972, t10982, t10985, t10987)
}
