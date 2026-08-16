//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 1745/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1745(t221: f64, t2379: f64, t4128: f64, t1489: f64, t9541: f64, t4126: f64, t782: f64) -> (f64, f64, f64) {
    let t13007 = t221 * t4128 * t2379;
    let t13010 = t9541 * t1489;
    let t13012 = t782 * t4126;
    (t13007, t13010, t13012)
}
