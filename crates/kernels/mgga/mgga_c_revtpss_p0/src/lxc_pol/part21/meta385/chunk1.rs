//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 1815/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1815<F: Float>(t1294: F, t3584: F, t1277: F, t12295: F, t12292: F, t12297: F, t12299: F, t12301: F, t12303: F, t12307: F, t12310: F, t12314: F, t12317: F, t12320: F) -> (F, F, F, F) {
    let t12606 = t3584 * t1294;
    let t12607 = t1277 * t12606;
    let t12610 = F::cast_from(0.46096296296296296297e-1_f64) * t12295;
    let t12621 = -t12610 + F::cast_from(0.19755555555555555556e-1_f64) * t12297 + F::cast_from(0.9877777777777777778e-2_f64) * t12299 - F::cast_from(0.29633333333333333334e-1_f64) * t12301 - F::cast_from(0.14816666666666666667e-1_f64) * t12303 + F::cast_from(0.16462962962962962963e-1_f64) * t12307 - F::cast_from(0.59266666666666666668e-1_f64) * t12310 - F::cast_from(0.29633333333333333334e-1_f64) * t12292 + F::cast_from(0.88900000000000000002e-1_f64) * t12314 + F::cast_from(0.88900000000000000002e-1_f64) * t12317 + F::cast_from(0.14816666666666666667e-1_f64) * t12320;
    (t12606, t12607, t12610, t12621)
}
