//! GGA_C_GAPLOC lxc pol — lxc_pol part 43 (v4rhosigma3_8) CSE chunk 906/1072 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part43_v4rhosigma3_8_chunk906(t43403: f64, t15499: f64, t28640: f64, t3487: f64, t9806: f64, t40966: f64, t2963: f64, t3295: f64, t9796: f64, t40969: f64, t1029: f64, t9829: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t43404 = 0.10352590007558602413e2_f64 * t43403;
    let t43407 = t28640 * t15499 * t3487 * t9806;
    let t43408 = 0.46011511144704899612e1_f64 * t43407;
    let t43409 = 0.11502877786176224903e1_f64 * t40966;
    let t43412 = t9796 * t2963 * t3295;
    let t43414 = 0.38342925953920749676e1_f64 * t40969;
    let t43416 = t9796 * t1029 * t9829;
    (t43404, t43408, t43409, t43412, t43414, t43416)
}
