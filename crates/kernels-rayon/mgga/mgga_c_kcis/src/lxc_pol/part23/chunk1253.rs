//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1253/1323 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk1253(t16789: f64, t51692: f64, t7923: f64, t27357: f64, t5721: f64, t94228: f64, t491: f64, t5747: f64, t1394: f64, t4165: f64, t28499: f64, t4173: f64) -> (f64, f64, f64, f64, f64) {
    let t98463 = t51692 * t7923 * t16789;
    let t98466 = t94228 * t5721 * t27357;
    let t98470 = t5747 * t491;
    let t98472 = t1394 * t98470 * t4165;
    let t98475 = t1394 * t28499 * t4173;
    (t98463, t98466, t98470, t98472, t98475)
}
