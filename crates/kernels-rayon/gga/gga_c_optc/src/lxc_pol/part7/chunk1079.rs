//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1079/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1079(t6569: f64, t732: f64, t737: f64, t2193: f64, t104: f64, t1975: f64, t22497: f64, t22562: f64, t22578: f64, t22581: f64, t22593: f64, t22685: f64, t22687: f64, t22690: f64, t22694: f64, t22697: f64, t95: f64) -> (f64, f64, f64) {
    let t23413 = 1820.0_f64 / 27.0_f64 * t732 * t6569;
    let t23414 = t737 * t6569;
    let t23417 = t2193 * t2193;
    let t23422 = -0.77534644304710291488e-2_f64 * t95 * t104 * t23417 * t1975 - t22685 + t22687 - t22690 - t22694 - t22497 + t22562 + t22578 + t22581 - t22593 + t22697;
    (t23413, t23414, t23422)
}
