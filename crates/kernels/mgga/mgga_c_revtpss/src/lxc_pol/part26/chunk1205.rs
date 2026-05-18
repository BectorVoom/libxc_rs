//! MGGA_C_REVTPSS lxc pol — lxc_pol part 26 (v4rho3sigma_1) CSE chunk 1205/1225 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part26_v4rho3sigma_1_chunk1205<F: Float>(t2470: F, t26270: F, t7284: F, t96220: F, t9675: F, t94771: F, t7514: F, t9288: F, t7289: F, t26277: F, t94776: F, t25950: F, t26292: F) -> (F, F, F, F, F, F, F, F) {
    let t96276 = t26270 * t2470;
    let t96277 = t7284 * t96276;
    let t96279 = t96220 * t9675;
    let t96280 = t94771 * t96279;
    let t96282 = t7514 * t9288;
    let t96284 = F::new(0.39982213492741449076e-1) * t7289 * t96282;
    let t96287 = t94776 * t26277;
    let t96289 = t25950 * t26292;
    (t96276, t96277, t96279, t96280, t96282, t96284, t96287, t96289)
}
