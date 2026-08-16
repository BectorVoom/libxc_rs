//! GGA_C_GAPLOC lxc pol — lxc_pol part 53 (v4rhosigma3_18) CSE chunk 267/1072 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part53_v4rhosigma3_18_chunk267(t2021: f64, t789: f64, t1: f64, t1858: f64, t787: f64, t1423: f64, t734: f64) -> (f64, f64, f64) {
    let t2022 = t2021 * t789;
    let t2026 = t1858 * t1;
    let t2027 = t787 * t2026;
    let t2028 = t1423 * t734;
    (t2022, t2027, t2028)
}
