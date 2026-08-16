//! GGA_C_GAPLOC lxc pol — lxc_pol part 43 (v4rhosigma3_8) CSE chunk 943/1072 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part43_v4rhosigma3_8_chunk943(t24295: f64, t3263: f64, t13241: f64, t5552: f64, t3073: f64, t5559: f64, t1960: f64, t3322: f64, t8440: f64, t27229: f64, t9777: f64, t10805: f64, t7324: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t44223 = 2.0_f64 * t24295 * t3263;
    let t44225 = 2.0_f64 * t5552 * t13241;
    let t44228 = 6.0_f64 * t5559 * t3073 * t3263;
    let t44231 = 2.0_f64 * t1960 * t3073 * t3322;
    let t44232 = t8440 * t3322;
    let t44234 = 6.0_f64 * t27229 * t9777;
    let t44236 = 4.0_f64 * t7324 * t10805;
    (t44223, t44225, t44228, t44231, t44232, t44234, t44236)
}
