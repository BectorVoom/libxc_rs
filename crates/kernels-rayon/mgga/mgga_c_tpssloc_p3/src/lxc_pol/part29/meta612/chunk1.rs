//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 2052/2357 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2052(t2240: f64, t24503: f64, t33: f64, t39054: f64, t7245: f64, t50: f64, t9300: f64, t1240: f64, t3630: f64, t11588: f64, t2127: f64, t221: f64) -> (f64, f64, f64, f64, f64) {
    let t85524 = t2240 * t33 * t24503;
    let t85536 = t39054 * t7245;
    let t85539 = t50 * t9300;
    let t85628 = t1240 * t3630;
    let t85639 = t2127 * t221 * t11588;
    (t85524, t85536, t85539, t85628, t85639)
}
