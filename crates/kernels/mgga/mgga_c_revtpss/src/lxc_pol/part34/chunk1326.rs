//! MGGA_C_REVTPSS lxc pol — lxc_pol part 34 (v4rho3sigma_9) CSE chunk 1326/1341 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part34_v4rho3sigma_9_chunk1326<F: Float>(t1903: F, t6874: F, t108332: F, t108335: F, t108337: F, t25924: F, t25930: F, t25931: F, t27868: F, t6918: F, t7295: F, t7920: F, t86641: F, t94682: F, t94700: F, t94703: F, t97894: F, t97900: F, t97917: F, t97923: F, t97926: F, t97956: F) -> F {
    let t114621 = t6874 * t1903;
    let t114632 = -F::new(0.78062653693846795158e1) * t7295 * t25924 * t7920 * t6918 - F::new(0.19514881078765566037e-2) * t97894 + F::new(0.28912093960683998208e-1) * t97900 + t94682 + F::new(0.51405703062096148814e-2) * t97917 + F::new(0.51405703062096148814e-2) * t97923 - F::new(0.28912093960683998208e-1) * t97926 + t94700 - t94703 - F::new(0.26020884564615598386e1) * t25930 * t25931 * t114621 + F::new(0.21684070470512998656e-1) * t108332 - F::new(0.43368140941025997312e-1) * t108335 + F::new(0.77108554593144223218e-1) * t108337 + F::new(0.72280234901709995519e-3) * t97956 + F::new(0.13010442282307799193e1) * t27868 * t25931 * t86641;
    t114632
}
