//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 33 (v4rho3sigma_9) CSE chunk 1008/1415 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part33_v4rho3sigma_9_chunk1008(t1539: f64, t5878: f64, t3071: f64, t10930: f64, t20234: f64, t974: f64, t20217: f64, t998: f64, t10942: f64, t21510: f64, t4583: f64, t4582: f64) -> (f64, f64, f64, f64, f64) {
    let t21531 = t5878 * t1539;
    let t21532 = t3071 * t21531;
    let t21537 = t10930 * t20234;
    let t21538 = t974 * t21537;
    let t21541 = t998 * t20217;
    let t21542 = t974 * t21541;
    let t21545 = t10942 * t20234;
    let t21546 = t974 * t21545;
    let t21550 = t4583 * t21510;
    let t21551 = t4582 * t21550;
    (t21532, t21538, t21542, t21546, t21551)
}
