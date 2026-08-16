//! GGA_C_GAPLOC lxc pol — lxc_pol part 49 (v4rhosigma3_14) CSE chunk 1105/1217 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part49_v4rhosigma3_14_chunk1105(t2684: f64, t2685: f64, t47143: f64, t12213: f64, t2464: f64, t2465: f64, t13851: f64, t2013: f64, t43393: f64, t43398: f64, t43401: f64, t43404: f64, t43408: f64, t43409: f64, t47133: f64, t47137: f64, t47140: f64) -> f64 {
    let t47145 = t2684 * t2685 * t47143;
    let t47149 = t2684 * t2464 * t2465 * t12213;
    let t47151 = t2013 * t13851;
    let t47153 = 0.15337170381568299871e1_f64 * t43393 + t43398 - t43401 - t43404 + t43408 + 0.15337170381568299871e1_f64 * t47133 - 0.25561950635947166451e1_f64 * t47137 + 0.25561950635947166451e0_f64 * t47140 + 0.19171462976960374838e0_f64 * t47145 - 0.42603251059911944084e-1_f64 * t47149 - 0.19171462976960374838e0_f64 * t47151 + t43409;
    t47153
}
