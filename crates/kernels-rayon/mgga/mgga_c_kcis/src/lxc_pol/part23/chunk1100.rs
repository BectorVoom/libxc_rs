//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1100/1323 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk1100(t1489: f64, t2038: f64, t28503: f64, t1464: f64, t1014: f64, t8176: f64, t5649: f64, t7923: f64, t1394: f64, t5655: f64, t5663: f64, t4153: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t28504 = t2038 * t1489;
    let t28505 = t28503 * t28504;
    let t28506 = t1464 * t28505;
    let t28508 = t1014 * t8176;
    let t28510 = t7923 * t5649;
    let t28511 = t1394 * t28510;
    let t28513 = t7923 * t5655;
    let t28514 = t1394 * t28513;
    let t28516 = t7923 * t5663;
    let t28517 = t4153 * t28516;
    (t28504, t28505, t28506, t28508, t28510, t28511, t28513, t28514, t28516, t28517)
}
