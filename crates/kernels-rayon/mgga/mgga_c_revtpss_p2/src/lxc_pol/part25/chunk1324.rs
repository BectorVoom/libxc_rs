//! MGGA_C_REVTPSS lxc pol — lxc_pol part 25 (v4rho3sigma_0) CSE chunk 1324/1360 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part25_v4rho3sigma_0_chunk1324(t2453: f64, t3908: f64, t7275: f64, t3923: f64, t7274: f64, t1399: f64, t2434: f64, t25880: f64, t25899: f64, t25885: f64, t25889: f64, t25921: f64, t26084: f64, t4078: f64, t4131: f64, t543: f64, t7295: f64, t7296: f64, t7301: f64, t7304: f64, t94591: f64, t94593: f64, t94598: f64, t94602: f64, t94605: f64, t94608: f64, t94610: f64, t94613: f64) -> (f64, f64, f64) {
    let t94616 = t2453 * t7275 * t3908;
    let t94628 = t7274 * t3923;
    let t94633 = t2434 * t1399;
    let t94634 = t25880 * t94633;
    let t94635 = t25899 * t94634;
    let t94637 = 0.13709901006661042888e-1_f64 * t94591 + 0.51405703062096148812e-1_f64 * t94593 - 0.86736281882051994623e-1_f64 * t94598 + t94602 - 0.43368140941025997312e-1_f64 * t94605 - t94608 + 0.13010442282307799193e1_f64 * t94610 * t7304 + 0.77108554593144223218e-1_f64 * t94613 + 0.34697458558045176417e-2_f64 * t94616 + 0.39512695097613069591e1_f64 * t26084 * t4078 + 0.26020884564615598386e1_f64 * t25921 * t25885 + 0.52041769129231196772e1_f64 * t25921 * t25889 + 0.26020884564615598386e1_f64 * t7295 * t7296 * t7274 * t4131 + 0.13010442282307799193e1_f64 * t7295 * t7301 * t94628 * t543 - 0.51405703062096148812e-1_f64 * t94635;
    (t94628, t94634, t94637)
}
