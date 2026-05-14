//! MGGA_C_REVTPSS lxc pol — lxc_pol part 26 (v4rho3sigma_1) CSE chunk 1074/1080 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part26_v4rho3sigma_1_chunk1074<F: Float>(t7284: F, t96370: F, t26234: F, t94886: F, t1398: F, t1445: F, t2097: F, t213: F, t225: F, t25921: F, t26079: F, t26246: F, t26333: F, t26343: F, t4003: F, t543: F, t561: F, t7295: F, t7301: F, t96362: F, t96405: F, t96510: F, t96512: F, t96516: F, t96527: F, t96542: F, t96546: F, t96549: F, t9890: F) -> (F,) {
    let t96550 = t7284 * t96370;
    let t96552 = t94886 * t26234;
    let t96554 = -0.51405703062096148814e-2 * t96510 - 0.19756347548806534796e1 * t96512 * t1445 - 0.34697458558045176417e-2 * t96516 + 0.65854491829355115987e0 * t213 * t96362 * t225 * t561 + 0.4336814094102599731e0 * t7295 * t7301 * t2097 * t9890 * t543 + 0.43368140941025997312e-1 * t96527 - 0.26020884564615598386e1 * t7295 * t26079 * t96405 * t4003 - 0.26020884564615598386e1 * t25921 * t26343 + 0.13010442282307799193e1 * t7295 * t7301 * t26333 * t1398 * t543 + 0.13010442282307799193e1 * t25921 * t26246 - 0.43368140941025997312e-1 * t96542 + 0.14456046980341999104e-2 * t96546 + t96549 + 0.21684070470512998656e-1 * t96550 + 0.15421710918628844643e0 * t96552;
    (t96554,)
}
