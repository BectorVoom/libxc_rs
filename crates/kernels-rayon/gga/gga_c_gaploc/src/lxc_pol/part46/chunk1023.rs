//! GGA_C_GAPLOC lxc pol — lxc_pol part 46 (v4rhosigma3_11) CSE chunk 1023/1029 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part46_v4rhosigma3_11_chunk1023(t11125: f64, t1960: f64, t977: f64, t2595: f64, t33992: f64, t13241: f64, t5559: f64, t841: f64, t24295: f64, t3263: f64, t5552: f64, t3073: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t44215 = t1960 * t11125 * t977;
    let t44217 = t33992 * t2595;
    let t44221 = 6.0_f64 * t5559 * t13241 * t841;
    let t44223 = 2.0_f64 * t24295 * t3263;
    let t44225 = 2.0_f64 * t5552 * t13241;
    let t44228 = 6.0_f64 * t5559 * t3073 * t3263;
    (t44215, t44217, t44221, t44223, t44225, t44228)
}
