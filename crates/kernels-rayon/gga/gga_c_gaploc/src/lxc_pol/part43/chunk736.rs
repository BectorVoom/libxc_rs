//! GGA_C_GAPLOC lxc pol — lxc_pol part 43 (v4rhosigma3_8) CSE chunk 736/1072 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part43_v4rhosigma3_8_chunk736(t14521: f64, t224: f64, t5558: f64, t744: f64, t1980: f64, t6110: f64, t124: f64, t1390: f64, t10928: f64, t1434: f64, t822: f64, t169: f64, t5750: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t14522 = t224 * t14521;
    let t14537 = t744 * t5558;
    let t15362 = t1980 * t6110;
    let t15481 = t124 * t1390;
    let t15498 = t822 * t10928 * t1434;
    let t15499 = t169 * t5750;
    (t14522, t14537, t15362, t15481, t15498, t15499)
}
