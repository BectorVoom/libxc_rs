//! GGA_C_GAPLOC lxc pol — lxc_pol part 39 (v4rhosigma3_4) CSE chunk 1119/1217 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part39_v4rhosigma3_4_chunk1119(t41139: f64, t1445: f64, t47225: f64, t833: f64, t47271: f64, t41143: f64, t43658: f64, t43661: f64, t43664: f64, t43666: f64, t43670: f64, t43674: f64, t43677: f64, t43680: f64) -> f64 {
    let t47283 = 0.76685851907841499354e0_f64 * t41139;
    let t47286 = t833 * t1445 * t47225;
    let t47290 = 0.11502877786176224903e2_f64 * t833 * t1445 * t47271;
    let t47293 = -t47283 + 0.76685851907841499354e0_f64 * t41143 + t43658 + t43661 + t43664 + 0.11502877786176224903e2_f64 * t47286 + t47290 - 0.79445533226334281487e-1_f64 * t43666 - t43670 - t43674 - 0.39722766613167140743e-1_f64 * t43677 - t43680;
    t47293
}
