//! MGGA_C_KCIS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 1163/1420 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part4_v3rho3_1_chunk1163(t14590: f64, t3438: f64, t3437: f64, t14706: f64, t5077: f64, t3337: f64, t10692: f64, t1801: f64, t10745: f64, t5073: f64, t1805: f64, t3425: f64) -> (f64, f64, f64, f64, f64) {
    let t14739 = t3438 * t14590;
    let t14740 = t3437 * t14739;
    let t14742 = t5077 * t14706;
    let t14743 = t3337 * t14742;
    let t14745 = t10692 * t1801;
    let t14747 = t10745 * t5073;
    let t14749 = t3425 * t1805;
    (t14740, t14743, t14745, t14747, t14749)
}
