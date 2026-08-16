//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 659/1222 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk659(t1882: f64, t5075: f64, t5153: f64, t5070: f64, t1131: f64, t2567: f64, t5064: f64, t258: f64, t4934: f64, t5053: f64, t5147: f64, t761: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t18544 = t1882 * t5075;
    let t18593 = t1882 * t5153;
    let t18633 = t1882 * t5070;
    let t18675 = t2567 * t1131;
    let t18680 = t2567 * t5064;
    let t18685 = t258 * t4934;
    let t18729 = t258 * t5053;
    let t18740 = t761 * t5147;
    (t18544, t18593, t18633, t18675, t18680, t18685, t18729, t18740)
}
