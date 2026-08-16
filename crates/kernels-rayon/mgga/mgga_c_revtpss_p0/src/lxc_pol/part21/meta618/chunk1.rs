//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2373/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2373(t10852: f64, t40336: f64, t10858: f64, t10863: f64, t10868: f64, t820: f64, t843: f64, t10874: f64, t2482: f64, t27: f64, t10872: f64, t221: f64, t2485: f64) -> (f64, f64, f64, f64) {
    let t40337 = t40336 * t10852;
    let t40345 = t10858 * t10863;
    let t40348 = t820 * t10868 * t843;
    let t40349 = t40348 * t10874;
    let t40352 = t2482 * t10868 * t27;
    let t40355 = t40352 * t2485 * t221 * t10872;
    (t40337, t40345, t40349, t40355)
}
