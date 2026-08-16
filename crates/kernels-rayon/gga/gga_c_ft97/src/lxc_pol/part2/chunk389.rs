//! GGA_C_FT97 lxc pol — lxc_pol part 2 (v3rho3_1) CSE chunk 389/1007 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part2_v3rho3_1_chunk389(t1970: f64, t2102: f64, t1792: f64, t582: f64, t1796: f64, t1984: f64, t2: f64, t1986: f64, t24: f64, t2075: f64, t586: f64, t2092: f64, t2093: f64, t2095: f64, t2098: f64, t462: f64, t92: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t2103 = t2102 * t1970;
    let t2106 = t582 * t1792;
    let t2109 = t582 * t1796;
    let t2112 = t1984 * t2;
    let t2114 = t24 * t2112 * t1986;
    let t2118 = t24 * t586 * t2075;
    let t2120 = t2092 + 2.0_f64 / 9.0_f64 * t2093 + 2.0_f64 / 3.0_f64 * t2095 - 2.0_f64 / 9.0_f64 * t462 * t2098 + 2.0_f64 / 3.0_f64 * t462 * t2103 + 2.0_f64 / 3.0_f64 * t462 * t2106 - t462 * t2109 / 3.0_f64 + 2.0_f64 * t92 * t2114 - t92 * t2118;
    (t2103, t2106, t2109, t2112, t2114, t2118, t2120)
}
