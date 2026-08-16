//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 3 (v3rho3_1) CSE chunk 1020/1255 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part3_v3rho3_1_chunk1020<F: Float>(t13203: F, t210: F, t4155: F, t9573: F, t2645: F, t2684: F, t4248: F, t13076: F, t13080: F, t13084: F, t13087: F, t13173: F, t13177: F, t13182: F, t13186: F, t13190: F, t13193: F, t13198: F, t13202: F, t2623: F, t2643: F, t2681: F, t4167: F, t4178: F, t4257: F, t787: F, t817: F, t831: F, t843: F, t9602: F, t9604: F) -> F {
    let t13204 = t210 * t13203;
    let t13208 = F::cast_from(7.0_f64) / F::cast_from(24.0_f64) * t9573 * t4155;
    let t13210 = t2645 * t4248 * t2684;
    let t13213 = -t2643 * t13076 / F::cast_from(3072.0_f64) - F::cast_from(5.0_f64) / F::cast_from(768.0_f64) * t2643 * t13080 - t4178 * t13084 / F::cast_from(384.0_f64) - F::cast_from(35.0_f64) / F::cast_from(216.0_f64) * t13087 - F::cast_from(119.0_f64) / F::cast_from(1728.0_f64) * t9602 + F::cast_from(7.0_f64) / F::cast_from(1152.0_f64) * t9604 + F::cast_from(5.0_f64) / F::cast_from(384.0_f64) * t2623 * t4257 - t817 * t13173 / F::cast_from(3072.0_f64) - t13177 * t831 / F::cast_from(1536.0_f64) - t4167 * t2681 / F::cast_from(3072.0_f64) - F::cast_from(119.0_f64) / F::cast_from(13824.0_f64) * t13182 - F::cast_from(5.0_f64) / F::cast_from(128.0_f64) * t843 * t13186 - t13190 + F::cast_from(5.0_f64) / F::cast_from(384.0_f64) * t843 * t13193 + F::cast_from(5.0_f64) / F::cast_from(768.0_f64) * t843 * t13198 + t13202 - t787 * t13204 / F::cast_from(48.0_f64) - t13208 + t2643 * t13210 / F::cast_from(768.0_f64);
    t13213
}
