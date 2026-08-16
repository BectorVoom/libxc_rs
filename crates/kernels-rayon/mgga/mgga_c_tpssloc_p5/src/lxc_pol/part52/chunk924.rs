//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 52 (v4rho2sigma2_8) CSE chunk 924/1400 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part52_v4rho2sigma2_8_chunk924(t22705: f64, t6978: f64, t22704: f64, t154: f64, t2558: f64, t1984: f64) -> (f64, f64, f64) {
    let t22706 = t22705 * t6978;
    let t22707 = t22704 * t22706;
    let t22715 = t2558 * t154;
    let t22716 = t22715 * t1984;
    (t22707, t22715, t22716)
}
