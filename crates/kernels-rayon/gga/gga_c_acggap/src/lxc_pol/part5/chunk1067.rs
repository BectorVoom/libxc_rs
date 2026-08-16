//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1067/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1067(t1210: f64, t524: f64, t377: f64, t4206: f64, t180: f64, t5079: f64, t1160: f64, t1539: f64, t1639: f64, t980: f64, t5319: f64, t310: f64, t5300: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t18906 = t1210 * t524;
    let t18910 = t377 * t4206;
    let t18912 = t180 * t5079;
    let t18914 = t1160 * t18912 * t1539;
    let t18916 = t980 * t1639;
    let t18918 = t377 * t5319;
    let t18920 = t310 * t5300;
    (t18906, t18910, t18914, t18916, t18918, t18920)
}
