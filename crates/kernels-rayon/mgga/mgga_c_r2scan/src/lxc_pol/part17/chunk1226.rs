//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 1226/1293 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk1226(t37835: f64, t37838: f64, t38528: f64, t38532: f64, t39679: f64, t39721: f64, t39723: f64, t39738: f64, t43215: f64, t43217: f64, t43219: f64, t43225: f64) -> f64 {
    let t44288 = -t39679 + 0.87327386630866483588e-2_f64 * t43215 - 0.97574405393827830187e-2_f64 * t43217 - 0.11565819519348392138e-2_f64 * t39721 + 0.32524801797942610062e-3_f64 * t39723 - 0.26198215989259945076e-1_f64 * t43219 + t38528 + t38532 + 0.58544643236296698113e-1_f64 * t37835 + 0.45022119329691164871e0_f64 * t37838 + t39738 + 0.69345773920434148507e0_f64 * t43225;
    t44288
}
