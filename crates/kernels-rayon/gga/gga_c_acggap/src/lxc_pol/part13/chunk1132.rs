//! GGA_C_ACGGAP lxc pol — lxc_pol part 13 (v4rho3sigma_5) CSE chunk 1132/1213 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part13_v4rho3sigma_5_chunk1132(t2001: f64, t4853: f64, t13287: f64, t31057: f64, t33953: f64, t5122: f64, t15386: f64, t35284: f64, t2302: f64, t4210: f64, t2260: f64, t7852: f64) -> (f64, f64, f64, f64, f64) {
    let t35545 = t2001 * t4853;
    let t35549 = t31057 * t13287 * t33953 * t5122;
    let t35550 = 0.62896184579208304136e-3_f64 * t35549;
    let t35552 = t31057 * t15386 * t35284;
    let t35553 = 0.94344276868812456204e-3_f64 * t35552;
    let t35556 = t31057 * t13287 * t2302 * t4210;
    let t35557 = 0.62896184579208304136e-3_f64 * t35556;
    let t35560 = t7852 * t2260;
    (t35545, t35550, t35553, t35557, t35560)
}
