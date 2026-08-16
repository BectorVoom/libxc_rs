//! GGA_C_GAPLOC lxc pol — lxc_pol part 27 (v4rho2sigma2_10) CSE chunk 1032/1468 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part27_v4rho2sigma2_10_chunk1032(t12290: f64, t12315: f64, t12317: f64, t12321: f64, t10285: f64, t10288: f64, t10289: f64, t10297: f64, t10300: f64, t11130: f64, t11132: f64, t11134: f64, t11137: f64, t11139: f64, t11140: f64, t12033: f64, t12149: f64, t12277: f64, t331: f64, t841: f64) -> (f64, f64) {
    let t12323 = t12290 + t12315 + t12317 + t12321;
    let t12325 = -t12277 * t841 + t12323 * t331 + t10285 + t10288 + t10289 + t10297 - t10300 - t11130 + t11132 + t11134 + t11137 + t11139 - t11140 + t12033 + t12149;
    (t12323, t12325)
}
