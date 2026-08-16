//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 52 (v4rho2sigma2_8) CSE chunk 429/1400 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part52_v4rho2sigma2_8_chunk429(t1834: f64, t539: f64, t1380: f64, t1825: f64, t553: f64, t1336: f64, t1814: f64, t544: f64, t564: f64) -> (f64, f64, f64, f64) {
    let t1835 = t539 * t1834;
    let t1838 = t1380 * t1825;
    let t1840 = t553 * t1834;
    let t1842 = -t1336 * t1838 + t1814 * t564 + t1840 * t544;
    (t1835, t1838, t1840, t1842)
}
