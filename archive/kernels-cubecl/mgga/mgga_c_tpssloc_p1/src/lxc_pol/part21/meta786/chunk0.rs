//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2726/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2726<F: Float>(t16398: F, t19890: F, t12283: F, t19972: F, t225: F, t56570: F, t16150: F, t16155: F, t16233: F, t16308: F, t16387: F, t16394: F, t19855: F, t19871: F, t19876: F, t19956: F, t3803: F, t3851: F, t3858: F, t40335: F, t40443: F, t40449: F, t5240: F, t5248: F, t54764: F, t54785: F, t54787: F, t54793: F, t54801: F, t54811: F, t554: F, t559: F) -> (F, F) {
    let t57450 = t16398 * t19890;
    let t57457 = t12283 * t19972;
    let t57465 = t56570 * t225;
    let t57481 = t16394 * t16308 / F::cast_from(192.0_f64) + F::cast_from(7.0_f64) / F::cast_from(144.0_f64) * t57450 - F::cast_from(7.0_f64) / F::cast_from(288.0_f64) * t54764 - t16233 * t5248 * t19871 * t40335 / F::cast_from(512.0_f64) + F::cast_from(7.0_f64) / F::cast_from(1152.0_f64) * t57457 - t19855 * t3858 / F::cast_from(3072.0_f64) + F::cast_from(5.0_f64) / F::cast_from(192.0_f64) * t5240 * t16150 + F::cast_from(5.0_f64) / F::cast_from(384.0_f64) * t5240 * t16155 + t57465 * t554 * t559 / F::cast_from(3072.0_f64) + F::cast_from(119.0_f64) / F::cast_from(3456.0_f64) * t54785 + F::cast_from(7.0_f64) / F::cast_from(72.0_f64) * t54787 + t19876 * t16387 / F::cast_from(256.0_f64) - F::cast_from(595.0_f64) / F::cast_from(5184.0_f64) * t54793 + F::cast_from(119.0_f64) / F::cast_from(13824.0_f64) * t40443 + t40449 - F::cast_from(7.0_f64) / F::cast_from(576.0_f64) * t54801 - t3803 * t5248 * t19956 * t3851 / F::cast_from(3072.0_f64) + F::cast_from(119.0_f64) / F::cast_from(1728.0_f64) * t54811;
    (t57465, t57481)
}
