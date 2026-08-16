//! GGA_C_GAPLOC lxc pol — lxc_pol part 39 (v4rhosigma3_4) CSE chunk 1031/1217 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part39_v4rhosigma3_4_chunk1031(t43199: f64, t4820: f64, t7513: f64, t2028: f64, t3038: f64, t787: f64, t9641: f64, t10999: f64, t2536: f64, t33565: f64, t7372: f64, t33294: f64, t9810: f64) -> (f64, f64, f64, f64, f64) {
    let t43670 = 0.79445533226334281487e-1_f64 * t7513 * t4820 * t43199;
    let t43674 = 0.39722766613167140743e-1_f64 * t787 * t9641 * t3038 * t2028;
    let t43677 = t787 * t2536 * t10999 * t2028;
    let t43679 = t33565 * t7372;
    let t43680 = 0.29792074959875355558e-1_f64 * t43679;
    let t43681 = t33294 * t9810;
    (t43670, t43674, t43677, t43680, t43681)
}
