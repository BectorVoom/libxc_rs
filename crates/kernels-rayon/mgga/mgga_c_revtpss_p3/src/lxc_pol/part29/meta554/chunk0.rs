//! MGGA_C_REVTPSS lxc pol — lxc_pol part 29 (v4rho3sigma_4) CSE chunk 1894/2049 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1894(t7284: f64, t96370: f64, t26234: f64, t94886: f64, t4132: f64, t689: f64, t7492: f64, t1445: f64, t2439: f64, t26358: f64, t26252: f64, t3920: f64) -> (f64, f64, f64, f64, f64) {
    let t96550 = t7284 * t96370;
    let t96552 = t94886 * t26234;
    let t96556 = t689 * t7492 * t4132;
    let t96559 = t2439 * t26358 * t1445;
    let t96561 = t26252 * t3920;
    (t96550, t96552, t96556, t96559, t96561)
}
