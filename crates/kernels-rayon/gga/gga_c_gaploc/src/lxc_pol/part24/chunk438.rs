//! GGA_C_GAPLOC lxc pol — lxc_pol part 24 (v4rho2sigma2_7) CSE chunk 438/1439 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part24_v4rho2sigma2_7_chunk438(t2095: f64, t313: f64, t191: f64, t325: f64, t107: f64, t121: f64, t830: f64) -> (f64, f64, f64, f64) {
    let t2096 = t313 * t2095;
    let t2097 = t191 * t325;
    let t2098 = t107 * t2097;
    let t2101 = t121 * t830;
    (t2096, t2097, t2098, t2101)
}
