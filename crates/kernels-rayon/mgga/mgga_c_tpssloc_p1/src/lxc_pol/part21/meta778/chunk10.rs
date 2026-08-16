//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2700/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2700(t12283: f64, t19976: f64, t19886: f64, t16257: f64, t16261: f64, t16305: f64, t16306: f64, t16311: f64, t19876: f64, t19956: f64, t19984: f64, t3803: f64, t3805: f64, t3856: f64, t5246: f64, t5248: f64, t5259: f64, t5287: f64, t54013: f64, t54086: f64, t54088: f64, t54090: f64, t54092: f64, t54114: f64, t54116: f64, t54118: f64, t54162: f64, t54165: f64, t54258: f64, t6394: f64) -> f64 {
    let t56837 = t12283 * t19976;
    let t56853 = t12283 * t19886;
    let t56866 = -7.0_f64 / 288.0_f64 * t54086 - 7.0_f64 / 576.0_f64 * t54088 + 7.0_f64 / 1152.0_f64 * t54090 + 7.0_f64 / 2304.0_f64 * t54092 - t3803 * t5248 * t19956 * t3856 / 3072.0_f64 + 7.0_f64 / 2304.0_f64 * t56837 - t5246 * t16305 * t16311 * t54165 / 192.0_f64 + 7.0_f64 / 1152.0_f64 * t54114 + t19876 * t16257 / 384.0_f64 + t19876 * t16261 / 768.0_f64 + t3803 * t16305 * t54258 * t6394 / 384.0_f64 - 7.0_f64 / 576.0_f64 * t54116 - 7.0_f64 / 288.0_f64 * t56853 - t3803 * t54013 * t16306 * t5287 / 768.0_f64 - 7.0_f64 / 576.0_f64 * t54118 + t54162 * t5259 / 192.0_f64 + t3803 * t3805 * t19984 * t3856 / 768.0_f64;
    t56866
}
