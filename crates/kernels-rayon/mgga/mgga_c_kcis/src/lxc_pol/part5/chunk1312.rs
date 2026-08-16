//! MGGA_C_KCIS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 1312/1419 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part5_v3rho3_2_chunk1312(t7176: f64, t743: f64, t7183: f64, t733: f64, t7167: f64, t738: f64, t7170: f64, t7173: f64, t7161: f64, t1330: f64, t21125: f64, t21130: f64, t3883: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t21721 = t743 * t7176;
    let t21723 = t733 * t7183;
    let t21725 = t738 * t7167;
    let t21727 = t738 * t7170;
    let t21729 = t743 * t7173;
    let t21731 = t733 * t7161;
    let t21734 = t1330 * t21125;
    let t21737 = t3883 * t21130;
    (t21721, t21723, t21725, t21727, t21729, t21731, t21734, t21737)
}
