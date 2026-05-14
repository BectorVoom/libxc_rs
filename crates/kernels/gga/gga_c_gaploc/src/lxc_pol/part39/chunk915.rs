//! GGA_C_GAPLOC lxc pol — lxc_pol part 39 (v4rhosigma3_4) CSE chunk 915/1028 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part39_v4rhosigma3_4_chunk915<F: Float>(t1365: F, t38281: F, t9074: F, t38277: F, t4261: F, t13749: F, t203: F, t550: F, t1358: F, t158: F, t123: F, t488: F, t13740: F, t2312: F, t42580: F, t42582: F, t42584: F, t42588: F, t42591: F, t46859: F) -> (F, F, F, F) {
    let t46862 = t9074 * t1365 * t38281;
    let t46865 = t9074 * t4261 * t38277;
    let t46867 = t203 * t13749;
    let t46868 = t550 * t46867;
    let t46871 = 0.31616674039640166221e-2 * t1358 * t1365 * t46868;
    let t46873 = t158 * t13749;
    let t46877 = 0.31616674039640166221e-2 * t1358 * t46873 * t123 * t488;
    let t46878 = t2312 * t13740;
    let t46880 = -t42580 + 0.11856252764865062333e-2 * t46859 - 0.35568758294595186999e-2 * t46862 + 0.23712505529730124666e-2 * t46865 + t46871 + t42582 + 0.23712505529730124666e-2 * t42584 - t42588 - t42591 - t46877 - 0.11856252764865062333e-2 * t46878;
    (t46867, t46868, t46873, t46880)
}
