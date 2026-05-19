//! GGA_C_GAPLOC lxc pol — lxc_pol part 49 (v4rhosigma3_14) CSE chunk 1074/1217 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part49_v4rhosigma3_14_chunk1074<F: Float>(t12068: F, t2268: F, t6320: F, t6509: F, t42594: F, t42597: F, t42601: F, t42602: F, t42603: F, t46884: F, t46887: F, t46889: F, t46892: F, t46896: F) -> F {
    let t46900 = t2268 * t6320 * t12068 * t6509;
    let t46902 = -F::cast_from(0.11856252764865062333e-2_f64) * t46884 - F::cast_from(0.11856252764865062333e-2_f64) * t46887 - t42594 + t42597 + t42601 + t42602 - t42603 + F::cast_from(0.11856252764865062333e-2_f64) * t46889 + F::cast_from(0.11856252764865062333e-2_f64) * t46892 + F::cast_from(0.34146007962811379518e0_f64) * t46896 - F::cast_from(0.17073003981405689759e0_f64) * t46900;
    t46902
}
