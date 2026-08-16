//! GGA_C_GAPLOC lxc pol — lxc_pol part 33 (v4rho2sigma2_16) CSE chunk 941/1464 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part33_v4rho2sigma2_16_chunk941(t10140: f64, t1445: f64, t597: f64, t10144: f64, t10241: f64, t4130: f64, t590: f64, t4781: f64, t9371: f64, t10268: f64, t4820: f64, t6824: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t10382 = t1445 * t10140;
    let t10384 = 0.11502877786176224903e2_f64 * t597 * t10382;
    let t10385 = t1445 * t10144;
    let t10387 = 0.11502877786176224903e2_f64 * t597 * t10385;
    let t10392 = t4130 * t10241 * t590;
    let t10394 = 0.15337170381568299871e1_f64 * t4781 * t10392;
    let t10395 = 0.15976219147466979032e-1_f64 * t9371;
    let t10396 = t4820 * t10268;
    let t10398 = 0.79445533226334281487e-1_f64 * t6824 * t10396;
    (t10382, t10384, t10385, t10387, t10392, t10394, t10395, t10396, t10398)
}
