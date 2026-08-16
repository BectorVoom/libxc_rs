//! GGA_C_GAPLOC lxc pol — lxc_pol part 21 (v4rho2sigma2_4) CSE chunk 414/1466 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part21_v4rho2sigma2_4_chunk414(t165: f64, t723: f64, t486: f64, t1967: f64, t1392: f64, t325: f64, t1391: f64, t1402: f64, t791: f64, t121: f64, t769: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t1968 = t165 * t723;
    let t1969 = t486 * t1968;
    let t1970 = t1967 * t1969;
    let t1973 = t1392 * t325;
    let t1974 = t1391 * t1973;
    let t1977 = t1402 * t791;
    let t1980 = t769 * t121;
    (t1968, t1969, t1970, t1974, t1977, t1980)
}
