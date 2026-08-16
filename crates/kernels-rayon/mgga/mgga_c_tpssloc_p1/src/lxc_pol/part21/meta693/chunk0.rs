//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2509/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2509(t13062: f64, t225: f64, t13378: f64, t193: f64, t2379: f64, t4331: f64, t591: f64, t2394: f64, t4344: f64) -> (f64, f64, f64, f64, f64) {
    let t47609 = t13062 * t225;
    let t47618 = t13378 * t225;
    let t47645 = t193 * t2379;
    let t47676 = 12.0_f64 * t4331 * t591;
    let t47705 = t2394 * t4344;
    (t47609, t47618, t47645, t47676, t47705)
}
