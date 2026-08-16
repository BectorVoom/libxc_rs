//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1286/1397 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk1286(t2104: f64, t27614: f64, t6176: f64, t6188: f64, t101894: f64, t27583: f64, t29532: f64, t4425: f64, t7978: f64, t1394: f64, t21898: f64, t7923: f64) -> (f64, f64, f64, f64) {
    let t102029 = t6176 * t27614 * t2104 * t6188;
    let t102032 = t27583 * t101894;
    let t102035 = t7978 * t4425 * t29532;
    let t102038 = t1394 * t7923 * t21898;
    (t102029, t102032, t102035, t102038)
}
