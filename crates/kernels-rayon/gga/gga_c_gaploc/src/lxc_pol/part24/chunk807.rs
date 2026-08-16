//! GGA_C_GAPLOC lxc pol — lxc_pol part 24 (v4rho2sigma2_7) CSE chunk 807/1439 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part24_v4rho2sigma2_7_chunk807(t2021: f64, t7634: f64, t2052: f64, t954: f64, t1880: f64, t2581: f64, t1445: f64, t2572: f64, t4614: f64, t7132: f64, t4752: f64, t740: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t7635 = t2021 * t7634;
    let t7638 = t2052 * t954;
    let t7643 = t2581 * t1880;
    let t7644 = t1445 * t7643;
    let t7647 = t4614 * t2572;
    let t7650 = t1445 * t7132;
    let t7653 = t4752 * t740;
    (t7635, t7638, t7644, t7647, t7650, t7653)
}
