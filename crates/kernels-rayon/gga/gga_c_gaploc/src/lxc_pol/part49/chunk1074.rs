//! GGA_C_GAPLOC lxc pol — lxc_pol part 49 (v4rhosigma3_14) CSE chunk 1074/1217 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part49_v4rhosigma3_14_chunk1074(t12068: f64, t2268: f64, t6320: f64, t6509: f64, t42594: f64, t42597: f64, t42601: f64, t42602: f64, t42603: f64, t46884: f64, t46887: f64, t46889: f64, t46892: f64, t46896: f64) -> f64 {
    let t46900 = t2268 * t6320 * t12068 * t6509;
    let t46902 = -0.11856252764865062333e-2_f64 * t46884 - 0.11856252764865062333e-2_f64 * t46887 - t42594 + t42597 + t42601 + t42602 - t42603 + 0.11856252764865062333e-2_f64 * t46889 + 0.11856252764865062333e-2_f64 * t46892 + 0.34146007962811379518e0_f64 * t46896 - 0.17073003981405689759e0_f64 * t46900;
    t46902
}
