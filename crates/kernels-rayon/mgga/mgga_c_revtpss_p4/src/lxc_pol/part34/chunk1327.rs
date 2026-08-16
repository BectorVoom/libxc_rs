//! MGGA_C_REVTPSS lxc pol — lxc_pol part 34 (v4rho3sigma_9) CSE chunk 1327/1341 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part34_v4rho3sigma_9_chunk1327(t1882: f64, t543: f64, t6918: f64, t1903: f64, t6844: f64, t6862: f64, t108282: f64, t108369: f64, t108380: f64, t108389: f64, t25930: f64, t25931: f64, t27837: f64, t27909: f64, t27980: f64, t30096: f64, t30101: f64, t30105: f64, t6919: f64, t7926: f64, t94761: f64, t94784: f64, t97875: f64, t97985: f64, t98003: f64) -> f64 {
    let t114636 = t6918 * t1882 * t543;
    let t114640 = t6844 * t1903;
    let t114660 = t6862 * t1903;
    let t114664 = -0.26020884564615598386e1_f64 * t25930 * t25931 * t114636 - 0.26020884564615598386e1_f64 * t25930 * t25931 * t114640 + 0.77108554593144223218e-1_f64 * t108369 - 0.52041769129231196772e1_f64 * t25930 * t97875 * t30105 - t94761 + 0.13010442282307799193e1_f64 * t108282 * t7926 + 0.51405703062096148812e-1_f64 * t97985 + 0.21684070470512998656e-1_f64 * t108380 - 0.43368140941025997312e-1_f64 * t108389 - 0.72280234901709995519e-3_f64 * t98003 + 0.26020884564615598386e1_f64 * t27837 * t30101 + 0.13010442282307799193e1_f64 * t27837 * t30096 + t94784 - 0.19756347548806534796e1_f64 * t27909 * t6919 + 0.52041769129231196772e1_f64 * t25930 * t27980 * t114660;
    t114664
}
