//! GGA_C_GAPLOC lxc pol — lxc_pol part 36 (v4rhosigma3_1) CSE chunk 1025/1029 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part36_v4rhosigma3_1_chunk1025(t1960: f64, t3073: f64, t3322: f64, t8440: f64, t27229: f64, t9777: f64, t10805: f64, t7324: f64, t8862: f64, t9780: f64, t1052: f64, t29646: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t44231 = 2.0_f64 * t1960 * t3073 * t3322;
    let t44232 = t8440 * t3322;
    let t44234 = 6.0_f64 * t27229 * t9777;
    let t44236 = 4.0_f64 * t7324 * t10805;
    let t44238 = 4.0_f64 * t8862 * t9780;
    let t44239 = t29646 * t1052;
    (t44231, t44232, t44234, t44236, t44238, t44239)
}
