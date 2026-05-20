//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2438/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2438<F: Float>(t549: F, t240: F, t72: F, t1408: F, t2237: F, t2482: F, t3981: F, t1369: F, t9726: F, t1372: F, t546: F, t9801: F) -> (F, F, F, F, F, F) {
    let t46624 = t549 * t549;
    let t46625 = F::new(1.0) / t46624;
    let t46627 = t240 * t46625 * t72;
    let t46644 = t2482 * t1408 * t2237;
    let t46645 = t46644 * t3981;
    let t46651 = t9726 * t1369;
    let t46652 = t46651 * t1372;
    let t46670 = t9801 * t546;
    (t46627, t46644, t46645, t46651, t46652, t46670)
}
