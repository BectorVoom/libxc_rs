//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 1720/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1720(t26656: f64, t4182: f64, t7823: f64, t814: f64, t829: f64, t25293: f64, t25317: f64, t226: f64, t23187: f64, t25274: f64, t25285: f64, t25289: f64, t25301: f64, t25304: f64, t25308: f64, t25310: f64, t25314: f64, t25322: f64, t25326: f64, t26613: f64, t26654: f64, t4281: f64, t4291: f64, t7839: f64, t808: f64, t812: f64) -> (f64, f64, f64, f64, f64) {
    let t26657 = t26656 * t4182;
    let t26661 = t814 * t7823;
    let t26662 = t26661 * t829;
    let t26667 = 0.38381794893125283518e-1_f64 * t25293;
    let t26673 = 0.16449340668482264365e-1_f64 * t25317;
    let t26676 = t26656 * t829;
    let t26678 = -0.16449340668482264365e-1_f64 * t25274 + t26613 + t226 * t26654 + 2.0_f64 * t4281 * t26657 + 0.82246703342411321825e-2_f64 * t23187 - t812 * t26662 - 0.16449340668482264365e-1_f64 * t25285 + 0.3289868133696452873e-1_f64 * t25289 + t808 * t7839 - t26667 + 0.3289868133696452873e-1_f64 * t25301 + 0.3289868133696452873e-1_f64 * t25304 - 0.3289868133696452873e-1_f64 * t25308 + 0.76763589786250567037e-1_f64 * t25310 - 0.3289868133696452873e-1_f64 * t25314 + t26673 - 0.3289868133696452873e-1_f64 * t25322 + 0.16449340668482264365e-1_f64 * t25326 - t4291 * t26676;
    (t26657, t26661, t26662, t26676, t26678)
}
