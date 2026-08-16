//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2781/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2781(t22236: f64, t808: f64, t9736: f64, t6884: f64, t9741: f64, t14104: f64, t47856: f64, t13729: f64, t2782: f64, t556: f64, t5774: f64, t2439: f64, t3895: f64, t6896: f64) -> (f64, f64, f64, f64, f64) {
    let t74714 = t9736 * t808 * t22236;
    let t74717 = t9741 * t6884;
    let t74733 = t47856 * t14104;
    let t74744 = t2782 * t556 * t13729 * t5774;
    let t74757 = t2439 * t3895 * t6896;
    (t74714, t74717, t74733, t74744, t74757)
}
