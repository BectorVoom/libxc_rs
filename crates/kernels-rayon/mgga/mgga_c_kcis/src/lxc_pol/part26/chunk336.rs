//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 336/1397 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk336(t1983: f64, t557: f64, t303: f64, t1471: f64, t1472: f64, t1650: f64, t1477: f64, t1897: f64) -> (f64, f64, f64, f64) {
    let t1984 = t1983 * t557;
    let t1985 = t303 * t1984;
    let t1988 = t1471 * t1472 * t1650;
    let t1991 = t1477 * t1897;
    (t1984, t1985, t1988, t1991)
}
