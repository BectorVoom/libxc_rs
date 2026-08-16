//! MGGA_C_R2SCAN lxc pol — lxc_pol part 15 (v4rho3sigma_5) CSE chunk 994/1253 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part15_v4rho3sigma_5_chunk994(t20: f64, t5119: f64, t3293: f64, t2124: f64, t7406: f64, t10760: f64, t7619: f64, t6093: f64, t7624: f64, t2147: f64, t3344: f64, t980: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t11669 = t5119 * t20;
    let t11670 = t3293 * t11669;
    let t11671 = t2124 * t7406;
    let t11672 = t11670 * t11671;
    let t11675 = t10760 * t7619;
    let t11676 = t6093 * t11675;
    let t11678 = t10760 * t7624;
    let t11679 = t2147 * t11678;
    let t11681 = t980 * t3344;
    (t11669, t11670, t11671, t11672, t11675, t11676, t11678, t11679, t11681)
}
