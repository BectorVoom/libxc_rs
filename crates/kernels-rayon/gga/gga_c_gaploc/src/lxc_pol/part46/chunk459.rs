//! GGA_C_GAPLOC lxc pol — lxc_pol part 46 (v4rhosigma3_11) CSE chunk 459/1029 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part46_v4rhosigma3_11_chunk459(t1564: f64, t911: f64, t1415: f64, t4390: f64, t191: f64, t599: f64, t588: f64) -> (f64, f64, f64, f64) {
    let t6915 = t911 * t1564;
    let t6963 = t1415 * t4390;
    let t6964 = t191 * t599;
    let t6985 = t588 * t599;
    (t6915, t6963, t6964, t6985)
}
