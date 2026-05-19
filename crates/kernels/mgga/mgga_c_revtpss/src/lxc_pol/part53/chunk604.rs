//! MGGA_C_REVTPSS lxc pol — lxc_pol part 53 (v4rho2sigma2_8) CSE chunk 604/1244 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part53_v4rho2sigma2_8_chunk604<F: Float>(t459: F, t5215: F, t1208: F, t1769: F, t487: F, t1770: F, t1214: F, t1774: F, t1211: F, t1294: F, t1277: F, t3358: F, t3579: F, t5044: F, t5049: F, t5054: F, t5058: F) -> (F, F, F, F, F, F, F, F) {
    let t5216 = t5215 * t459;
    let t5219 = t1769 * t1208;
    let t5220 = t5219 * t487;
    let t5225 = t1770 * t487;
    let t5230 = t1774 * t1214;
    let t5231 = t1211 * t5230;
    let t5236 = t1774 * t1294;
    let t5237 = t1277 * t5236;
    let t5245 = t3579 - F::cast_from(0.4938888888888888889e-2_f64) * t3358 - F::cast_from(0.4938888888888888889e-2_f64) * t5044 - F::cast_from(0.9877777777777777778e-2_f64) * t5049 + F::cast_from(0.29633333333333333334e-1_f64) * t5054 + F::cast_from(0.14816666666666666667e-1_f64) * t5058;
    (t5216, t5219, t5220, t5225, t5230, t5231, t5237, t5245)
}
