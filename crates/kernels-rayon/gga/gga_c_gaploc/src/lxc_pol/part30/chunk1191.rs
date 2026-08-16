//! GGA_C_GAPLOC lxc pol — lxc_pol part 30 (v4rho2sigma2_13) CSE chunk 1191/1436 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part30_v4rho2sigma2_13_chunk1191(t2268: f64, t2304: f64, t27082: f64, t10253: f64, t484: f64, t10246: f64, t6305: f64, t6447: f64, t8195: f64, t1366: f64, t2755: f64, t10242: f64, t1595: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t31928 = 0.19918504644973304719e0_f64 * t2268 * t2304 * t27082;
    let t31929 = t484 * t10253;
    let t31930 = 0.31616674039640166222e-2_f64 * t31929;
    let t31932 = 0.39837009289946609438e0_f64 * t6305 * t10246;
    let t31935 = 0.39837009289946609438e0_f64 * t2268 * t6447 * t8195;
    let t31936 = t2755 * t1366;
    let t31939 = 0.39837009289946609438e0_f64 * t2268 * t2304 * t31936;
    let t31942 = 0.28455006635676149599e-1_f64 * t2268 * t1595 * t10242;
    (t31928, t31930, t31932, t31935, t31939, t31942)
}
