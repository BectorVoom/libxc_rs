//! GGA_C_GAPLOC lxc pol — lxc_pol part 21 (v4rho2sigma2_4) CSE chunk 1223/1466 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part21_v4rho2sigma2_4_chunk1223(t32697: f64, t11135: f64, t5552: f64, t2728: f64, t8440: f64, t16705: f64, t3459: f64, t24303: f64, t977: f64, t10805: f64, t5559: f64, t841: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t32698 = 0.96131577876777803547e-3_f64 * t32697;
    let t32708 = 4.0_f64 * t5552 * t11135;
    let t32713 = 2.0_f64 * t8440 * t2728;
    let t32715 = 2.0_f64 * t16705 * t3459;
    let t32716 = t24303 * t977;
    let t32719 = 12.0_f64 * t5559 * t10805 * t841;
    (t32698, t32708, t32713, t32715, t32716, t32719)
}
