//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 932/1222 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk932(t55558: f64, t55562: f64, t5419: f64, t8232: f64, t2842: f64, t5374: f64, t5395: f64, t848: f64, t38953: f64, t5410: f64, t5399: f64, t2252: f64, t342: f64, t5202: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t72080 = 56.0_f64 / 81.0_f64 * t55558;
    let t72082 = 56.0_f64 / 243.0_f64 * t55562;
    let t72167 = t8232 * t5419;
    let t72231 = t5374 * t2842;
    let t72263 = t8232 * t5395;
    let t72391 = t848 * t5374;
    let t72523 = t38953 * t5410;
    let t72805 = t8232 * t5399;
    let t72977 = t342 * t2252 * t5202;
    (t72080, t72082, t72167, t72231, t72263, t72391, t72523, t72805, t72977)
}
