//! GGA_C_GAPLOC lxc pol — lxc_pol part 33 (v4rho2sigma2_16) CSE chunk 1184/1464 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part33_v4rho2sigma2_16_chunk1184(t10429: f64, t1358: f64, t2299: f64, t488: f64, t2268: f64, t27102: f64, t6316: f64, t10249: f64, t6313: f64, t31590: f64, t426: f64, t535: f64) -> (f64, f64, f64, f64) {
    let t31998 = 0.63233348079280332442e-2_f64 * t1358 * t2299 * t10429 * t488;
    let t32001 = 0.14227503317838074799e1_f64 * t2268 * t6316 * t27102;
    let t32003 = 0.91056021234163678716e0_f64 * t6313 * t10249;
    let t32005 = t31590 * t426;
    let t32008 = 0.56910013271352299198e-1_f64 * t2268 * t535 * t32005;
    (t31998, t32001, t32003, t32008)
}
