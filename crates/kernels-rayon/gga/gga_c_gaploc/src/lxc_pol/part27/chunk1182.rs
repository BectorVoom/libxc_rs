//! GGA_C_GAPLOC lxc pol — lxc_pol part 27 (v4rho2sigma2_10) CSE chunk 1182/1468 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part27_v4rho2sigma2_10_chunk1182(t31889: f64, t2268: f64, t6320: f64, t6509: f64, t8097: f64, t20117: f64, t2854: f64, t10246: f64, t6313: f64, t123: f64, t25760: f64, t2326: f64, t9074: f64) -> (f64, f64, f64, f64, f64) {
    let t31890 = 0.11856252764865062333e-2_f64 * t31889;
    let t31894 = 0.34146007962811379518e0_f64 * t2268 * t6320 * t8097 * t6509;
    let t31898 = 0.34146007962811379518e0_f64 * t2268 * t6320 * t2854 * t20117;
    let t31900 = 0.53116012386595479252e0_f64 * t6313 * t10246;
    let t31903 = t25760 * t123;
    let t31905 = t9074 * t31903 * t2326;
    (t31890, t31894, t31898, t31900, t31905)
}
