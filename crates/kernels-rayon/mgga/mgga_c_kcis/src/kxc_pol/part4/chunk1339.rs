//! MGGA_C_KCIS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 1339/1420 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_kxc_pol_part4_v3rho3_1_chunk1339(t17063: f64, t17252: f64, t509: f64, t552: f64, t557: f64, t303: f64, t1014: f64, t5872: f64, t1928: f64, t4161: f64, t4165: f64, t4160: f64) -> (f64, f64, f64, f64, f64) {
    let t17253 = t17063 + t17252;
    let t17254 = t509 * t17253;
    let t17255 = t17254 * t552;
    let t17256 = t17255 * t557;
    let t17257 = t303 * t17256;
    let t17259 = t1014 * t5872;
    let t17260 = 0.33163888888888888888e-2_f64 * t17259;
    let t17261 = t4161 * t1928;
    let t17262 = t17261 * t4165;
    let t17263 = t4160 * t17262;
    (t17253, t17257, t17259, t17260, t17263)
}
