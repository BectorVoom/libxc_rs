//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1447/1798 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1447<F: Float>(t141: F, t2908: F, t41325: F, t41310: F, t930: F, t41318: F, t9303: F, t931: F, t41308: F, t41312: F, t41320: F, t41327: F, t41330: F, t41332: F, t41334: F, t41336: F, t41365: F, t41367: F) -> (F, F, F, F, F) {
    let t41433 = t141 * t2908 * t41325;
    let t41436 = t141 * t930 * t41310;
    let t41439 = t141 * t930 * t41318;
    let t41441 = t9303 * t931;
    let t41443 = -F::new(0.24154e1) * t41365 + F::cast_from(0.80513333333333333333e0_f64) * t41367 + F::new(0.24154e1) * t41308 + F::new(0.72462e1) * t41312 + F::new(0.181155e1) * t41320 - F::cast_from(0.60384999999999999999e0_f64) * t41327 - F::cast_from(0.80513333333333333336e0_f64) * t41330 - F::cast_from(0.53675555555555555556e0_f64) * t41332 + F::cast_from(0.40256666666666666668e0_f64) * t41334 + F::cast_from(0.44729629629629629629e0_f64) * t41336 - F::new(0.82785e-1) * t41433 + F::new(0.198684e1) * t41436 + F::new(0.49671e0) * t41439 + F::cast_from(0.98115555555555555556e0_f64) * t41441;
    (t41433, t41436, t41439, t41441, t41443)
}
