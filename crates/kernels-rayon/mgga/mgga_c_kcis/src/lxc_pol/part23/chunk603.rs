//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 603/1323 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk603(t1396: f64, t5477: f64, t1395: f64, t5780: f64, t1951: f64, t532: f64, t833: f64, t1409: f64, t1650: f64, t1419: f64, t167: f64, t518: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t5781 = t1396 * t5477;
    let t5782 = t1395 * t5781;
    let t5783 = t5780 * t5782;
    let t5787 = t532 * t1951;
    let t5789 = t1951 * t833;
    let t5792 = t1409 * t1650;
    let t5793 = t5792 * t1419;
    let t5796 = t518 * t167;
    (t5781, t5782, t5783, t5787, t5789, t5792, t5793, t5796)
}
