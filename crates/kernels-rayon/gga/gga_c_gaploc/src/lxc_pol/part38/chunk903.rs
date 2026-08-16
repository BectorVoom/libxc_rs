//! GGA_C_GAPLOC lxc pol — lxc_pol part 38 (v4rhosigma3_3) CSE chunk 903/1003 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part38_v4rhosigma3_3_chunk903(t43467: f64, t43470: f64, t10889: f64, t1457: f64, t326: f64, t43476: f64, t43502: f64, t43519: f64, t44878: f64, t45315: f64, t45319: f64, t45323: f64, t45326: f64, t45329: f64, t45331: f64, t45335: f64, t45337: f64, t45343: f64, t45349: f64, t45350: f64, t45356: f64, t45357: f64, t701: f64, t7572: f64, t7573: f64, t7584: f64, t7585: f64, t825: f64) -> f64 {
    let t45358 = 0.11916829983950142223e0_f64 * t43467;
    let t45359 = 0.11916829983950142223e0_f64 * t43470;
    let t45363 = -0.71500979903700853338e0_f64 * t10889 * t1457 * t44878 * t701 + t45315 + t45319 - t45323 + t45326 + 0.12780975317973583226e0_f64 * t45329 + 0.95857314884801874192e0_f64 * t45331 - 0.21301625529955972043e0_f64 * t45335 - 0.23005755572352449806e2_f64 * t7584 * t7585 * t45337 - t45343 + 0.13803453343411469884e2_f64 * t7572 * t7573 * t45337 + t45349 - 0.18404604457881959845e2_f64 * t825 * t326 * t45350 - t45356 + t45357 + t45358 + t45359 - 0.12780975317973583226e0_f64 * t43476 + 0.59584149919750711116e-1_f64 * t43502 + 0.11916829983950142223e0_f64 * t43519;
    t45363
}
