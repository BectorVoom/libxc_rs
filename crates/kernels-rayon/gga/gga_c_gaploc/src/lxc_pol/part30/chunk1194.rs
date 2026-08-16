//! GGA_C_GAPLOC lxc pol — lxc_pol part 30 (v4rho2sigma2_13) CSE chunk 1194/1436 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part30_v4rho2sigma2_13_chunk1194(t1217: f64, t3344: f64, t10273: f64, t4141: f64, t20369: f64, t2268: f64, t24139: f64, t8124: f64, t3808: f64, t1358: f64, t3394: f64, t488: f64, t6540: f64) -> (f64, f64, f64, f64, f64) {
    let t31973 = t1217 * t3344;
    let t31974 = 0.36886119712913527259e-2_f64 * t31973;
    let t31984 = 0.31616674039640166222e-2_f64 * t4141 * t10273;
    let t31988 = 0.68292015925622759036e0_f64 * t2268 * t24139 * t8124 * t20369;
    let t31990 = 0.63233348079280332442e-2_f64 * t3808 * t10273;
    let t31994 = 0.63233348079280332442e-2_f64 * t1358 * t6540 * t3394 * t488;
    (t31974, t31984, t31988, t31990, t31994)
}
