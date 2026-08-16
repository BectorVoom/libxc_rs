//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1162/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1162(t13802: f64, t4054: f64, t16894: f64, t2367: f64, t999: f64, t15016: f64, t4539: f64, t15012: f64, t14863: f64, t4536: f64, t1220: f64, t17634: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t52264 = t4054 * t13802;
    let t52269 = t999 * t2367 * t16894;
    let t52312 = t15016 * t4539;
    let t52314 = t15012 * t4539;
    let t52316 = t4536 * t14863;
    let t52319 = t1220 * t2367 * t17634;
    (t52264, t52269, t52312, t52314, t52316, t52319)
}
