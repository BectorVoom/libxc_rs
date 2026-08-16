//! GGA_C_GAPLOC lxc pol — lxc_pol part 33 (v4rho2sigma2_16) CSE chunk 807/1464 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part33_v4rho2sigma2_16_chunk807(t1865: f64, t7290: f64, t2365: f64, t6111: f64, t7221: f64, t723: f64, t1445: f64, t1710: f64, t2571: f64, t2541: f64, t769: f64, t313: f64, t7143: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t7718 = t7290 * t1865;
    let t7719 = t2365 * t7718;
    let t7720 = t6111 * t7719;
    let t7722 = t7221 * t723;
    let t7723 = t1445 * t7722;
    let t7726 = t2571 * t1710;
    let t7727 = t1445 * t7726;
    let t7730 = t769 * t2541;
    let t7733 = t313 * t7143;
    (t7720, t7722, t7723, t7727, t7730, t7733)
}
