//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2395/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2395(t10817: f64, t14379: f64, t10655: f64, t14389: f64, t13655: f64, t2792: f64, t912: f64, t2904: f64, t4446: f64, t10523: f64, t1573: f64, t10629: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t49090 = 12.0_f64 * t10817 * t14379;
    let t49092 = 0.96491876992155210402e2_f64 * t10655 * t14389;
    let t49095 = 6.0_f64 * t2792 * t13655 * t912;
    let t49096 = t4446 * t2904;
    let t49099 = t1573 * t10523;
    let t49104 = t1573 * t10629;
    (t49090, t49092, t49095, t49096, t49099, t49104)
}
