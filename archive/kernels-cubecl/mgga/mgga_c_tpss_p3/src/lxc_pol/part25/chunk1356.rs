//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1356/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1356<F: Float>(t18006: F, t18770: F, t19736: F, t19762: F, t19767: F, t19781: F, t20463: F, t20466: F, t20482: F, t20494: F, t21312: F, t21627: F, t3721: F, t5571: F, t5572: F, t61222: F, t61226: F, t62671: F, t6337: F, t64060: F, t66362: F, t66559: F, t70030: F, t70039: F, t70042: F, t70046: F, t70060: F, t70063: F, t70074: F, t70094: F, t70103: F, t70113: F, t70123: F, t70130: F, t70134: F) -> F {
    let t72026 = F::cast_from(2.0_f64) * t19767 * t66362 * t19781 - F::cast_from(2.0_f64) * t18006 * t18770 * t70123 + F::cast_from(6.0_f64) * t19767 * t66559 * t70042 - F::cast_from(6.0_f64) * t19767 * t20482 * t70046 - F::cast_from(4.0_f64) * t18006 * t66362 * t19762 - F::cast_from(4.0_f64) * t64060 * t20466 + F::cast_from(4.0_f64) * t5571 * t5572 * t6337 * t3721 + t19767 * t18770 * t70060 - F::cast_from(2.0_f64) * t18006 * t18770 * t70063 + F::cast_from(2.0_f64) * t19767 * t18770 * t70130 + t19767 * t18770 * t70134 + F::cast_from(6.0_f64) * t61226 * t18770 * t70103 - F::cast_from(4.0_f64) * t61222 * t21627 - F::cast_from(4.0_f64) * t18006 * t62671 * t21312 - F::cast_from(4.0_f64) * t18006 * t18770 * t70113 + F::cast_from(2.0_f64) * t70039 * t20494 - F::cast_from(4.0_f64) * t18006 * t18770 * t70030 + F::cast_from(4.0_f64) * t18006 * t20482 * t70074 - F::cast_from(4.0_f64) * t19767 * t20482 * t70094 - F::cast_from(12.0_f64) * t19736 * t20463;
    t72026
}
