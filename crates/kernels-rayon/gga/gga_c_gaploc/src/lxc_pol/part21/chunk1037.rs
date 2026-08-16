//! GGA_C_GAPLOC lxc pol — lxc_pol part 21 (v4rho2sigma2_4) CSE chunk 1037/1466 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part21_v4rho2sigma2_4_chunk1037(t121: f64, t5745: f64, t2084: f64, t321: f64, t2088: f64, t324: f64, t1953: f64, t1959: f64, t304: f64, t330: f64, t5557: f64, t679: f64, t8: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t16534 = t121 * t5745;
    let t16687 = t2084 * t321;
    let t16692 = 1.0_f64 / t2088 / t324;
    let t16705 = t1953 * t1959;
    let t16710 = t304 / t5557 / t330;
    let t16788 = t8 * t679;
    (t16534, t16687, t16692, t16705, t16710, t16788)
}
