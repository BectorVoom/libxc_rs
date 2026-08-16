//! MGGA_C_KCIS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 1141/1419 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part5_v3rho3_2_chunk1141(t1662: f64, t4972: f64, t2894: f64, t6521: f64, t9938: f64, t991: f64, t1003: f64, t6326: f64, t9933: f64, t18653: f64, t4939: f64, t14492: f64, t18648: f64) -> (f64, f64, f64, f64, f64) {
    let t19189 = t1662 * t4972;
    let t19190 = t2894 * t19189;
    let t19193 = t9938 * t6521;
    let t19194 = t991 * t19193;
    let t19196 = t6326 * t1003;
    let t19197 = t9933 * t19196;
    let t19200 = t4939 * t18653;
    let t19203 = t14492 * t18648;
    (t19190, t19194, t19197, t19200, t19203)
}
