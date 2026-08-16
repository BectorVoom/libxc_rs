//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 54 (v4rho2sigma2_10) CSE chunk 997/1484 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part54_v4rho2sigma2_10_chunk997(t25319: f64, t776: f64, t6637: f64, t6552: f64, t1894: f64, t4265: f64, t214: f64, t1880: f64, t1909: f64, t226: f64, t23187: f64, t25277: f64, t25281: f64, t25285: f64, t25289: f64, t25293: f64, t25295: f64, t25297: f64, t25301: f64, t25304: f64, t25308: f64, t25310: f64, t25314: f64, t25317: f64, t4162: f64, t4166: f64, t4281: f64, t6658: f64, t7535: f64, t808: f64, t812: f64) -> (f64, f64, f64) {
    let t25320 = t25319 * t776;
    let t25321 = t6637 * t25320;
    let t25322 = t6552 * t25321;
    let t25324 = t1894 * t4265;
    let t25325 = t214 * t25324;
    let t25326 = t1880 * t25325;
    let t25328 = 0.19190897446562641759e-1_f64 * t25277 - t4166 * t6658 + 0.41123351671205660912e-2_f64 * t23187 + 2.0_f64 * t4281 * t25281 - 0.82246703342411321825e-2_f64 * t25285 + 0.16449340668482264365e-1_f64 * t25289 + t808 * t7535 + t4162 * t1909 - 0.19190897446562641759e-1_f64 * t25293 + t226 * t25295 - t812 * t25297 + 0.16449340668482264365e-1_f64 * t25301 + 0.16449340668482264365e-1_f64 * t25304 - 0.16449340668482264365e-1_f64 * t25308 + 0.38381794893125283518e-1_f64 * t25310 - 0.16449340668482264365e-1_f64 * t25314 + 0.82246703342411321825e-2_f64 * t25317 - 0.16449340668482264365e-1_f64 * t25322 + 0.82246703342411321825e-2_f64 * t25326;
    (t25322, t25326, t25328)
}
