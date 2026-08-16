//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 1710/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1710(t18442: f64, t18473: f64, t18535: f64, t18569: f64, t225: f64, t68: f64, t484: f64, t18215: f64, t3440: f64, t18211: f64, t1653: f64, t5012: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t18571 = t18442 + t18473 + t18535 + t18569;
    let t18572 = t18571 * t225;
    let t18573 = t18572 * t68;
    let t18574 = t18573 * t484;
    let t18577 = t3440 * t18215;
    let t18580 = t3440 * t18211;
    let t18583 = t5012 * t1653;
    (t18571, t18572, t18573, t18574, t18577, t18580, t18583)
}
