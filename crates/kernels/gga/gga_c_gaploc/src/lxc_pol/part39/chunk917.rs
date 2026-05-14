//! GGA_C_GAPLOC lxc pol — lxc_pol part 39 (v4rhosigma3_4) CSE chunk 917/1028 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part39_v4rhosigma3_4_chunk917<F: Float>(t42594: F, t42597: F, t42601: F, t42602: F, t42603: F, t46884: F, t46887: F, t46889: F, t46892: F, t46896: F, t46900: F, t1063: F, t11981: F, t2343: F, t6519: F, t13755: F, t2268: F, t535: F) -> (F, F, F) {
    let t46902 = -0.11856252764865062333e-2 * t46884 - 0.11856252764865062333e-2 * t46887 - t42594 + t42597 + t42601 + t42602 - t42603 + 0.11856252764865062333e-2 * t46889 + 0.11856252764865062333e-2 * t46892 + 0.34146007962811379518e0 * t46896 - 0.17073003981405689759e0 * t46900;
    let t46908 = t1063 * t2343 * t11981 * t6519;
    let t46912 = 0.28455006635676149599e-1 * t2268 * t535 * t13755;
    (t46902, t46908, t46912)
}
