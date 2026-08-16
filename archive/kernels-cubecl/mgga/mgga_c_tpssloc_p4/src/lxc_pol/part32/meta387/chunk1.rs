//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 1467/2369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1467<F: Float>(t120: F, t5527: F, t829: F, t9646: F, t5544: F, t2645: F, t16839: F, t2647: F, t13177: F, t13251: F, t13260: F, t13275: F, t13277: F, t13280: F, t13287: F, t13320: F, t13330: F, t1512: F, t16872: F, t16877: F, t16879: F, t16888: F, t16893: F, t2643: F, t4167: F, t4178: F, t4191: F, t4236: F, t4240: F, t4250: F, t831: F) -> (F, F, F, F) {
    let t16896 = t120 * t5527;
    let t16898 = t9646 * t16896 * t829;
    let t16901 = t120 * t5544;
    let t16903 = t2645 * t16901 * t829;
    let t16907 = t2645 * t16839 * t2647;
    let t16910 = -t4167 * t4236 / F::cast_from(1536.0_f64) - t16872 * t831 / F::cast_from(3072.0_f64) - t13177 * t1512 / F::cast_from(1536.0_f64) + F::cast_from(7.0_f64) / F::cast_from(2304.0_f64) * t16877 - F::cast_from(7.0_f64) / F::cast_from(2304.0_f64) * t16879 - t13260 + t13275 + t13277 + t13280 - t13287 + t13251 * t4191 / F::cast_from(384.0_f64) - t13251 * t4240 / F::cast_from(1536.0_f64) + t13251 * t4250 / F::cast_from(384.0_f64) - F::cast_from(5.0_f64) / F::cast_from(384.0_f64) * t2643 * t16888 + t4178 * t16893 / F::cast_from(1536.0_f64) - F::cast_from(5.0_f64) / F::cast_from(768.0_f64) * t2643 * t16898 + t2643 * t16903 / F::cast_from(768.0_f64) + t13320 - t13330 + t2643 * t16907 / F::cast_from(768.0_f64);
    (t16898, t16903, t16907, t16910)
}
