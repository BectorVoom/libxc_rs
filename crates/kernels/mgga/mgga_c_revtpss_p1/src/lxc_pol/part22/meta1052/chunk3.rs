//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3716/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3716<F: Float>(t21143: F, t3636: F, t17448: F, t17580: F, t17679: F, t17684: F, t17690: F, t17732: F, t21014: F, t21017: F, t21242: F, t21267: F, t3620: F, t57128: F, t57145: F, t57164: F, t57167: F, t57170: F, t57344: F, t57707: F, t70303: F) -> F {
    let t70432 = t21143 * t3636;
    let t70453 = -F::cast_from(0.19055119163586549765e-3_f64) * t70432 - F::cast_from(0.25724410870841842184e-2_f64) * t57344 * t21267 - F::cast_from(0.2540682555144873302e-2_f64) * t21242 * t3620 + F::cast_from(0.3811023832717309953e-3_f64) * t57128 + F::cast_from(0.3811023832717309953e-3_f64) * t57145 + F::cast_from(0.11433071498151929859e-2_f64) * t70303 * t17732 + F::cast_from(0.45732285992607719436e-2_f64) * t57707 * t17580 - F::cast_from(0.3811023832717309953e-3_f64) * t57164 - F::cast_from(0.3811023832717309953e-3_f64) * t57167 - F::cast_from(0.19055119163586549765e-3_f64) * t57170 + F::cast_from(0.47637797908966374413e-3_f64) * t17448 * t17690 + F::cast_from(0.30488190661738479624e-2_f64) * t21014 * t17679 - F::cast_from(0.15244095330869239812e-2_f64) * t21017 * t17684;
    t70453
}
