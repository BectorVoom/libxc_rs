//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 772/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk772(t1976: f64, t5484: f64, t722: f64, t730: f64, t1975: f64, t712: f64) -> (f64, f64, f64) {
    let t5486 = t1976 * t5484 * t722;
    let t5488 = 0.35089341735807877242e1_f64 * t730 * t5486;
    let t5490 = 1.0_f64 / t1975 / t712;
    (t5486, t5488, t5490)
}
