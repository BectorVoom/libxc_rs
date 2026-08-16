//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2529/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2529<F: Float>(t10069: F, t14537: F, t10504: F, t136: F, t2457: F, t4533: F, t14473: F, t9303: F, t14477: F, t2435: F, t10073: F, t14482: F) -> (F, F, F, F, F) {
    let t51703 = t10069 * t14537;
    let t51704 = F::cast_from(0.21951497276451705329e-1_f64) * t51703;
    let t51726 = t10504 * t4533 * t136 * t2457;
    let t51727 = F::cast_from(0.34697458558045176417e-2_f64) * t51726;
    let t51733 = t9303 * t14473;
    let t51741 = t2435 * t14477;
    let t51742 = F::cast_from(0.21951497276451705329e-1_f64) * t51741;
    let t51756 = t10073 * t14482;
    (t51704, t51727, t51733, t51742, t51756)
}
