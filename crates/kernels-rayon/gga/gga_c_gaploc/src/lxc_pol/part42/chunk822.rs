//! GGA_C_GAPLOC lxc pol — lxc_pol part 42 (v4rhosigma3_7) CSE chunk 822/1012 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part42_v4rhosigma3_7_chunk822(t13310: f64, t2312: f64, t42825: f64, t1063: f64, t11259: f64, t6320: f64, t6519: f64, t2268: f64, t2854: f64, t31585: f64, t11413: f64, t24139: f64, t6509: f64) -> (f64, f64, f64, f64, f64) {
    let t44528 = t2312 * t13310;
    let t44529 = 0.11856252764865062333e-2_f64 * t44528;
    let t44530 = 0.12646669615856066489e-1_f64 * t42825;
    let t44534 = 0.17073003981405689759e0_f64 * t1063 * t6320 * t11259 * t6519;
    let t44538 = 0.34146007962811379518e0_f64 * t2268 * t6320 * t2854 * t31585;
    let t44542 = 0.68292015925622759036e0_f64 * t2268 * t24139 * t11413 * t6509;
    (t44529, t44530, t44534, t44538, t44542)
}
