//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2540/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2540(t10704: f64, t4395: f64, t2904: f64, t4446: f64, t10523: f64, t1573: f64, t10629: f64, t1556: f64, t2842: f64, t10702: f64, t10828: f64, t1580: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t49072 = t4395 * t10704;
    let t49096 = t4446 * t2904;
    let t49099 = t1573 * t10523;
    let t49104 = t1573 * t10629;
    let t49226 = t2842 * t1556;
    let t49240 = t10702 * t1556;
    let t49263 = t10828 * t1580;
    (t49072, t49096, t49099, t49104, t49226, t49240, t49263)
}
