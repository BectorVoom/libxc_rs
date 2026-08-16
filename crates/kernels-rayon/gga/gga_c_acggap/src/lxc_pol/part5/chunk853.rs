//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 853/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk853(t743: f64, t746: f64, t11735: f64, t11747: f64, t11756: f64, t11770: f64, t11775: f64, t11792: f64, t11825: f64, t11828: f64, t11834: f64, t11837: f64, t11869: f64, t11870: f64, t12015: f64, t256: f64, t257: f64, t2754: f64, t2755: f64, t2768: f64, t2795: f64, t62: f64, t680: f64, t690: f64, t728: f64, t739: f64, t745: f64, t747: f64, t748: f64, t75: f64, t793: f64, t799: f64) -> f64 {
    let t12083 = t743 * t743;
    let t12086 = t746 * t746;
    let t12091 = -24.0_f64 * t2768 * t12015 * t257 + 0.51947577317044391277e2_f64 * t799 * t11735 * t690 - t11747 + t11756 - 0.12304822629859687989e5_f64 * t75 * t11869 * t11870 * t2795 - t11770 + t11775 - t11792 - t11825 - t11828 - 0.11579025239058625248e4_f64 * t2768 * t748 * t739 + 0.21053605041484726346e2_f64 * t799 * t793 * t680 + 0.12865583598954028054e3_f64 * t745 * t2754 * t747 * t256 - 8.0_f64 * t728 * t2755 * t256 + t11834 - t11837 + 0.19964560303604640732e6_f64 * t62 / t12083 * t12015 / t12086;
    t12091
}
