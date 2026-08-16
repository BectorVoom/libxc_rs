//! GGA_C_ACGGAP lxc pol — lxc_pol part 13 (v4rho3sigma_5) CSE chunk 996/1213 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part13_v4rho3sigma_5_chunk996(t1614: f64, t7927: f64, t2138: f64, t2147: f64, t463: f64, t8418: f64, t30028: f64, t615: f64, t8790: f64, t929: f64, t157: f64, t33643: f64) -> (f64, f64, f64, f64, f64) {
    let t33715 = t7927 * t1614;
    let t33726 = 0.34694512752820797848e1_f64 * t2138 * t2147 * t8418 * t463;
    let t33727 = t615 * t30028;
    let t33735 = t8790 * t929;
    let t33739 = t33643 * t157;
    (t33715, t33726, t33727, t33735, t33739)
}
