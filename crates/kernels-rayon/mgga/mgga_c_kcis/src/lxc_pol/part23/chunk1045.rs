//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1045/1323 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk1045(t4137: f64, t553: f64, t303: f64, t1489: f64, t1494: f64, t1497: f64, t27387: f64, t1464: f64, t2642: f64, t5653: f64, t7923: f64, t1394: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t27419 = t553 * t4137;
    let t27420 = t303 * t27419;
    let t27423 = t1494 * t1489 * t1497;
    let t27424 = t27387 * t27423;
    let t27425 = t1464 * t27424;
    let t27427 = t5653 * t2642;
    let t27428 = t7923 * t27427;
    let t27429 = t1394 * t27428;
    (t27419, t27420, t27423, t27424, t27425, t27427, t27428, t27429)
}
