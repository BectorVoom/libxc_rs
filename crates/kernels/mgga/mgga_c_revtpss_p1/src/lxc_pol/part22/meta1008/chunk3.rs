//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3451/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3451<F: Float>(t225: F, t64816: F, t15648: F, t1651: F, t3133: F, t6244: F, t42078: F, t51967: F, t63274: F, t63276: F, t63278: F, t63281: F, t63285: F, t63290: F, t63293: F, t63299: F, t63304: F, t63308: F) -> (F, F, F, F) {
    let t64907 = t64816 * t225;
    let t64912 = t1651 * t15648;
    let t64916 = t6244 * t3133;
    let t64945 = F::cast_from(0.59266666666666666668e-1_f64) * t63274 - F::cast_from(0.19755555555555555556e-1_f64) * t63276 + F::cast_from(0.65851851851851851854e-2_f64) * t63278 - F::cast_from(0.19755555555555555556e-1_f64) * t63281 - F::cast_from(0.9877777777777777778e-2_f64) * t63285 - F::cast_from(0.16462962962962962963e-1_f64) * t63290 + F::cast_from(0.59266666666666666668e-1_f64) * t63293 + F::cast_from(0.29633333333333333334e-1_f64) * t63299 + F::cast_from(0.19755555555555555556e0_f64) * t63304 - F::cast_from(0.35560000000000000001e0_f64) * t63308 + t42078 + F::cast_from(0.9877777777777777778e-2_f64) * t51967;
    (t64907, t64912, t64916, t64945)
}
