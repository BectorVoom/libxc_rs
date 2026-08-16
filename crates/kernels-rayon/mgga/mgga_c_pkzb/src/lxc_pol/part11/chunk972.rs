//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 972/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk972(t10589: f64, t10590: f64, t10595: f64, t10601: f64, t158: f64, t10502: f64, t5356: f64, t2632: f64, t3396: f64, t10556: f64, t596: f64, t1029: f64, t1031: f64, t160: f64, t162: f64, t2631: f64, t3431: f64, t3435: f64, t3438: f64) -> (f64, f64, f64, f64, f64) {
    let t10604 = (t10589 + t10590 + t10595 + t10601) * t158;
    let t10612 = t5356 * t10502;
    let t10615 = t2632 * t3396;
    let t10618 = t596 * t10556;
    let t10621 = -36.0_f64 * t1029 * t3435 + 9.0_f64 * t1029 * t3438 + 9.0_f64 * t1031 * t3431 - t10604 * t162 + 60.0_f64 * t10612 * t160 - 36.0_f64 * t10615 * t2631 + 3.0_f64 * t10618 * t160;
    (t10604, t10612, t10615, t10618, t10621)
}
