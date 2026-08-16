//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2604/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2604(t13726: f64, t9303: f64, t13725: f64, t1445: f64, t2439: f64, t14082: f64, t3920: f64, t14078: f64, t2470: f64, t3915: f64, t13735: f64, t2435: f64) -> (f64, f64, f64, f64, f64) {
    let t47938 = t9303 * t13726;
    let t47942 = t2439 * t13725 * t1445;
    let t47944 = t14082 * t3920;
    let t47945 = 0.39029762157531132076e-1_f64 * t47944;
    let t47947 = t3915 * t14078 * t2470;
    let t47948 = 0.39029762157531132076e-1_f64 * t47947;
    let t47952 = t2435 * t13735;
    (t47938, t47942, t47945, t47948, t47952)
}
