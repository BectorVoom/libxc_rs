//! MGGA_C_TPSSLOC kxc pol — kxc_pol part 3 (v3rho3_1) CSE chunk 1023/1255 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_kxc_pol_part3_v3rho3_1_chunk1023<F: Float>(t13258: F, t4184: F, t242: F, t9972: F, t812: F, t2631: F, t9975: F, t4180: F, t4181: F, t13225: F, t13231: F, t13234: F, t13237: F, t13244: F, t13248: F, t13251: F, t13254: F, t2643: F, t2649: F, t4178: F, t4191: F, t4240: F, t9639: F, t9642: F, t9668: F, t9672: F, t9675: F, t9679: F, t9986: F, t9988: F, t9994: F) -> (F, F) {
    let t13260 = F::cast_from(7.0_f64) / F::cast_from(1152.0_f64) * t13258 * t4184;
    let t13261 = t9972 * t242;
    let t13262 = t812 * t13261;
    let t13263 = t9975 * t2631;
    let t13265 = t4180 * t4181 * t13263;
    let t13268 = -F::cast_from(7.0_f64) / F::cast_from(576.0_f64) * t9639 - F::cast_from(7.0_f64) / F::cast_from(2304.0_f64) * t9668 - F::cast_from(119.0_f64) / F::cast_from(6912.0_f64) * t9672 + F::cast_from(7.0_f64) / F::cast_from(2304.0_f64) * t9675 + F::cast_from(7.0_f64) / F::cast_from(4608.0_f64) * t9679 + F::cast_from(7.0_f64) / F::cast_from(4608.0_f64) * t9986 - F::cast_from(35.0_f64) / F::cast_from(1152.0_f64) * t9988 + F::cast_from(7.0_f64) / F::cast_from(576.0_f64) * t9994 + t2643 * t13225 / F::cast_from(384.0_f64) - t4178 * t13231 / F::cast_from(192.0_f64) + F::cast_from(119.0_f64) / F::cast_from(13824.0_f64) * t13234 - t13237 + t9642 * t4191 / F::cast_from(384.0_f64) - t9642 * t4240 / F::cast_from(1536.0_f64) + t4178 * t13244 / F::cast_from(768.0_f64) + t4178 * t13248 / F::cast_from(1536.0_f64) + t13251 * t2649 / F::cast_from(384.0_f64) + t13254 * t4184 / F::cast_from(768.0_f64) - t13260 - t13262 * t13265 / F::cast_from(512.0_f64);
    (t13263, t13268)
}
