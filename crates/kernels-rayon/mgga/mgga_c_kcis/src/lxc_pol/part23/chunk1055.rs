//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1055/1323 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk1055(t4124: f64, t6028: f64, t27520: f64, t4262: f64, t7948: f64, t3954: f64, t4136: f64, t5909: f64, t7952: f64, t3722: f64, t4261: f64, t11776: f64, t585: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t27521 = t6028 * t4124;
    let t27522 = t27520 * t27521;
    let t27524 = t7948 * t4262;
    let t27526 = t6028 * t3954;
    let t27527 = t7948 * t27526;
    let t27529 = t5909 * t4136;
    let t27530 = t7952 * t27529;
    let t27532 = t4261 * t3722;
    let t27533 = t7952 * t27532;
    let t27535 = t11776 * t585;
    (t27521, t27522, t27524, t27526, t27527, t27529, t27530, t27532, t27533, t27535)
}
