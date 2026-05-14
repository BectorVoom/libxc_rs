//! MGGA_C_REVTPSS lxc pol — lxc_pol part 36 (v4rho3sigma_11) CSE chunk 1191/1228 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part36_v4rho3sigma_11_chunk1191<F: Float>(t1882: F, t543: F, t6918: F, t1903: F, t6844: F, t6862: F, t108282: F, t108369: F, t108380: F, t108389: F, t25930: F, t25931: F, t27837: F, t27909: F, t27980: F, t30096: F, t30101: F, t30105: F, t6919: F, t7926: F, t94761: F, t94784: F, t97875: F, t97985: F, t98003: F) -> (F,) {
    let t114636 = t6918 * t1882 * t543;
    let t114640 = t6844 * t1903;
    let t114660 = t6862 * t1903;
    let t114664 = -0.26020884564615598386e1 * t25930 * t25931 * t114636 - 0.26020884564615598386e1 * t25930 * t25931 * t114640 + 0.77108554593144223218e-1 * t108369 - 0.52041769129231196772e1 * t25930 * t97875 * t30105 - t94761 + 0.13010442282307799193e1 * t108282 * t7926 + 0.51405703062096148812e-1 * t97985 + 0.21684070470512998656e-1 * t108380 - 0.43368140941025997312e-1 * t108389 - 0.72280234901709995519e-3 * t98003 + 0.26020884564615598386e1 * t27837 * t30101 + 0.13010442282307799193e1 * t27837 * t30096 + t94784 - 0.19756347548806534796e1 * t27909 * t6919 + 0.52041769129231196772e1 * t25930 * t27980 * t114660;
    (t114664,)
}
