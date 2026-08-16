//! MGGA_C_REVTPSS lxc pol — lxc_pol part 29 (v4rho3sigma_4) CSE chunk 1988/2049 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1988(t1385: f64, t8085: f64, t1903: f64, t26304: f64, t25930: f64, t25933: f64, t27864: f64, t27868: f64, t27972: f64, t28911: f64, t28915: f64, t48025: f64, t94705: f64, t94823: f64, t96392: f64, t96549: f64, t96550: f64, t96552: f64, t96556: f64, t96559: f64, t96561: f64, t96564: f64, t96565: f64) -> f64 {
    let t102656 = t1385 * t8085;
    let t102661 = t26304 * t1903;
    let t102669 = -0.17347256376410398924e1_f64 * t94705 * t28915 - 0.17347256376410398924e1_f64 * t25930 * t96392 * t27972 + t96549 + 0.72280234901709995518e-2_f64 * t96550 - 0.8673628188205199462e0_f64 * t27868 * t28911 * t48025 - 0.17347256376410398924e1_f64 * t25930 * t96392 * t27864 - 0.17347256376410398924e1_f64 * t25930 * t102656 * t25933 + 0.51405703062096148812e-1_f64 * t96552 + 0.52041769129231196772e1_f64 * t94823 * t102661 * t25933 + 0.54878743191129263322e-2_f64 * t96556 + 0.13009920719177044025e-2_f64 * t96559 - 0.2601984143835408805e-1_f64 * t96561 - t96564 + 0.38549458614245330943e-1_f64 * t96565;
    t102669
}
