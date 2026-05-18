//! GGA_C_GAPLOC lxc pol — lxc_pol part 49 (v4rhosigma3_14) CSE chunk 1072/1217 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part49_v4rhosigma3_14_chunk1072<F: Float>(t123: F, t1358: F, t46873: F, t488: F, t13740: F, t2312: F, t42580: F, t42582: F, t42584: F, t42588: F, t42591: F, t46859: F, t46862: F, t46865: F, t46871: F) -> F {
    let t46877 = F::new(0.31616674039640166221e-2) * t1358 * t46873 * t123 * t488;
    let t46878 = t2312 * t13740;
    let t46880 = -t42580 + F::new(0.11856252764865062333e-2) * t46859 - F::new(0.35568758294595186999e-2) * t46862 + F::new(0.23712505529730124666e-2) * t46865 + t46871 + t42582 + F::new(0.23712505529730124666e-2) * t42584 - t42588 - t42591 - t46877 - F::new(0.11856252764865062333e-2) * t46878;
    t46880
}
