//! GGA_C_GAPLOC lxc pol — lxc_pol part 39 (v4rhosigma3_4) CSE chunk 1014/1217 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part39_v4rhosigma3_4_chunk1014(t43400: f64, t33308: f64, t9805: f64, t9806: f64, t15499: f64, t28640: f64, t3487: f64, t40966: f64, t2963: f64, t3295: f64, t9796: f64, t40969: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t43401 = 0.15337170381568299871e1_f64 * t43400;
    let t43403 = t9805 * t33308 * t9806;
    let t43404 = 0.10352590007558602413e2_f64 * t43403;
    let t43407 = t28640 * t15499 * t3487 * t9806;
    let t43408 = 0.46011511144704899612e1_f64 * t43407;
    let t43409 = 0.11502877786176224903e1_f64 * t40966;
    let t43412 = t9796 * t2963 * t3295;
    let t43413 = 0.76685851907841499353e0_f64 * t43412;
    let t43414 = 0.38342925953920749676e1_f64 * t40969;
    (t43401, t43404, t43408, t43409, t43413, t43414)
}
