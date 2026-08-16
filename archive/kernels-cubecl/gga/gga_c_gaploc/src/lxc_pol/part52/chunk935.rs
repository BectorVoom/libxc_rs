//! GGA_C_GAPLOC lxc pol — lxc_pol part 52 (v4rhosigma3_17) CSE chunk 935/1013 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part52_v4rhosigma3_17_chunk935<F: Float>(t13343: F, t17293: F, t605: F, t2321: F, t38674: F, t9074: F, t1365: F, t38281: F, t38277: F, t4261: F, t13740: F, t2312: F) -> (F, F, F, F, F) {
    let t46835 = F::cast_from(24.0_f64) * t17293 * t13343 * t605;
    let t46859 = t9074 * t38674 * t2321;
    let t46862 = t9074 * t1365 * t38281;
    let t46865 = t9074 * t4261 * t38277;
    let t46878 = t2312 * t13740;
    (t46835, t46859, t46862, t46865, t46878)
}
