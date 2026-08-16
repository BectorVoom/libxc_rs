//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2422/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2422(t10828: f64, t300: f64, t4475: f64, t49514: f64, t10753: f64, t4488: f64, t959: f64, t14480: f64, t2940: f64, t2930: f64, t1581: f64, t13716: f64, t2904: f64, t952: f64) -> (f64, f64, f64, f64, f64) {
    let t49532 = t300 * t10828;
    let t49535 = 0.31168546390226634766e3_f64 * t49532 * t4475 * t49514;
    let t49538 = 0.11696447245269292414e1_f64 * t959 * t4488 * t10753;
    let t49540 = 0.10526802520742363173e2_f64 * t2940 * t14480;
    let t49541 = t300 * t2930;
    let t49544 = 0.10526802520742363173e2_f64 * t49541 * t1581 * t49514;
    let t49548 = 0.35089341735807877242e1_f64 * t959 * t2904 * t13716 * t952;
    (t49535, t49538, t49540, t49544, t49548)
}
