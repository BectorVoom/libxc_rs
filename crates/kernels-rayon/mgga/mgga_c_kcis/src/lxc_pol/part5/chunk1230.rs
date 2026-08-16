//! MGGA_C_KCIS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 1230/1419 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part5_v3rho3_2_chunk1230(t1262: f64, t6326: f64, t11020: f64, t18653: f64, t5302: f64, t15227: f64, t18648: f64, t15231: f64, t18657: f64, t1662: f64, t5336: f64, t3515: f64) -> (f64, f64, f64, f64, f64) {
    let t20600 = t6326 * t1262;
    let t20601 = t11020 * t20600;
    let t20604 = t5302 * t18653;
    let t20607 = t15227 * t18648;
    let t20610 = t15231 * t18657;
    let t20613 = t1662 * t5336;
    let t20614 = t3515 * t20613;
    (t20601, t20604, t20607, t20610, t20614)
}
