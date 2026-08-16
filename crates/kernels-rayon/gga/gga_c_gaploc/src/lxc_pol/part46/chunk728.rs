//! GGA_C_GAPLOC lxc pol — lxc_pol part 46 (v4rhosigma3_11) CSE chunk 728/1029 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part46_v4rhosigma3_11_chunk728(t121: f64, t5745: f64, t2084: f64, t321: f64, t2088: f64, t324: f64, t304: f64, t330: f64, t5557: f64, t123: f64, t160: f64, t4348: f64, t498: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t16534 = t121 * t5745;
    let t16687 = t2084 * t321;
    let t16692 = 1.0_f64 / t2088 / t324;
    let t16710 = t304 / t5557 / t330;
    let t16879 = t2084 * t123;
    let t16880 = t16879 * t160;
    let t17288 = t498 * t4348;
    (t16534, t16687, t16692, t16710, t16879, t16880, t17288)
}
