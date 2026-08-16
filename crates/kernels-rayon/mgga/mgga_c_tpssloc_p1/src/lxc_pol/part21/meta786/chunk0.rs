//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2726/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2726(t16398: f64, t19890: f64, t12283: f64, t19972: f64, t225: f64, t56570: f64, t16150: f64, t16155: f64, t16233: f64, t16308: f64, t16387: f64, t16394: f64, t19855: f64, t19871: f64, t19876: f64, t19956: f64, t3803: f64, t3851: f64, t3858: f64, t40335: f64, t40443: f64, t40449: f64, t5240: f64, t5248: f64, t54764: f64, t54785: f64, t54787: f64, t54793: f64, t54801: f64, t54811: f64, t554: f64, t559: f64) -> (f64, f64) {
    let t57450 = t16398 * t19890;
    let t57457 = t12283 * t19972;
    let t57465 = t56570 * t225;
    let t57481 = t16394 * t16308 / 192.0_f64 + 7.0_f64 / 144.0_f64 * t57450 - 7.0_f64 / 288.0_f64 * t54764 - t16233 * t5248 * t19871 * t40335 / 512.0_f64 + 7.0_f64 / 1152.0_f64 * t57457 - t19855 * t3858 / 3072.0_f64 + 5.0_f64 / 192.0_f64 * t5240 * t16150 + 5.0_f64 / 384.0_f64 * t5240 * t16155 + t57465 * t554 * t559 / 3072.0_f64 + 119.0_f64 / 3456.0_f64 * t54785 + 7.0_f64 / 72.0_f64 * t54787 + t19876 * t16387 / 256.0_f64 - 595.0_f64 / 5184.0_f64 * t54793 + 119.0_f64 / 13824.0_f64 * t40443 + t40449 - 7.0_f64 / 576.0_f64 * t54801 - t3803 * t5248 * t19956 * t3851 / 3072.0_f64 + 119.0_f64 / 1728.0_f64 * t54811;
    (t57465, t57481)
}
