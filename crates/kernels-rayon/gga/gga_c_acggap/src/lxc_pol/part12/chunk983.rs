//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 983/1250 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk983(t2138: f64, t322: f64, t8004: f64, t8107: f64, t1264: f64, t2147: f64, t2229: f64, t15407: f64, t7942: f64, t9427: f64, t31935: f64, t7963: f64) -> (f64, f64, f64, f64) {
    let t33128 = t2138 * t8004 * t8107 * t322;
    let t33132 = t2138 * t2147 * t2229 * t1264;
    let t33138 = t7942 * t9427 * t15407;
    let t33144 = t7963 * t9427 * t31935;
    (t33128, t33132, t33138, t33144)
}
