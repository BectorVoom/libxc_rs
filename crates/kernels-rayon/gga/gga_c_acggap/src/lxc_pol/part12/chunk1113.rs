//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 1113/1250 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk1113(t4680: f64, t7426: f64, t8605: f64, t30468: f64, t4916: f64, t31346: f64, t4419: f64, t15386: f64, t31195: f64, t35749: f64, t17912: f64, t2288: f64, t31443: f64, t3169: f64) -> (f64, f64, f64, f64, f64) {
    let t35797 = t7426 * t4680 * t8605;
    let t35799 = t30468 * t4916;
    let t35801 = t31346 * t4419;
    let t35804 = t31195 * t15386 * t35749;
    let t35808 = t31443 * t17912 * t2288 * t3169;
    (t35797, t35799, t35801, t35804, t35808)
}
