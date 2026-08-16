//! GGA_C_FT97 lxc pol — lxc_pol part 2 (v3rho3_1) CSE chunk 384/1007 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part2_v3rho3_1_chunk384(t2075: f64, t526: f64, t27: f64, t89: f64, t1957: f64, t1960: f64, t1963: f64, t1967: f64, t1972: f64, t1977: f64, t1981: f64, t1989: f64) -> (f64, f64, f64) {
    let t2076 = t526 * t2075;
    let t2078 = t89 * t27 * t2076;
    let t2080 = t1957 + t1960 + t1963 - t1967 / 27.0_f64 + t1972 / 9.0_f64 + t1977 / 9.0_f64 - t1981 / 18.0_f64 + t1989 / 3.0_f64 - t2078 / 6.0_f64;
    (t2076, t2078, t2080)
}
