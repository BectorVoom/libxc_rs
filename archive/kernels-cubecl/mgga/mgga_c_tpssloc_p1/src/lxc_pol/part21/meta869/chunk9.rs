//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 3191/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3191<F: Float>(t15438: F, t15548: F, t15569: F, t15608: F, t15689: F, t4889: F, t1174: F, t135: F, t18996: F, t11665: F, t15650: F, t18969: F, t3440: F, t45197: F, t5005: F, t52704: F, t52897: F, t53064: F, t53067: F, t53079: F, t53093: F, t53096: F, t53099: F, t53102: F, t53176: F, t63315: F) -> F {
    let t66255 = t15438 * t15548;
    let t66268 = t15569 * t15608;
    let t66273 = t4889 * t15689;
    let t66276 = t1174 * t135 * t18996;
    let t66282 = -t66255 / F::cast_from(1152.0_f64) - t53064 / F::cast_from(1728.0_f64) + F::cast_from(5.0_f64) / F::cast_from(10368.0_f64) * t53067 + t45197 * t52897 * t52704 * t53176 / F::cast_from(128.0_f64) + t1174 * t3440 * t63315 / F::cast_from(216.0_f64) - t11665 * t18969 / F::cast_from(2304.0_f64) + t66268 / F::cast_from(324.0_f64) + t53079 / F::cast_from(5184.0_f64) + t53093 / F::cast_from(384.0_f64) + t53096 / F::cast_from(162.0_f64) + t66273 / F::cast_from(81.0_f64) - t66276 / F::cast_from(432.0_f64) + t53099 / F::cast_from(5184.0_f64) - t53102 / F::cast_from(576.0_f64) - t5005 * t15650 / F::cast_from(576.0_f64);
    t66282
}
