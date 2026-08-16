//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3451/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3451(t225: f64, t64816: f64, t15648: f64, t1651: f64, t3133: f64, t6244: f64, t42078: f64, t51967: f64, t63274: f64, t63276: f64, t63278: f64, t63281: f64, t63285: f64, t63290: f64, t63293: f64, t63299: f64, t63304: f64, t63308: f64) -> (f64, f64, f64, f64) {
    let t64907 = t64816 * t225;
    let t64912 = t1651 * t15648;
    let t64916 = t6244 * t3133;
    let t64945 = 0.59266666666666666668e-1_f64 * t63274 - 0.19755555555555555556e-1_f64 * t63276 + 0.65851851851851851854e-2_f64 * t63278 - 0.19755555555555555556e-1_f64 * t63281 - 0.9877777777777777778e-2_f64 * t63285 - 0.16462962962962962963e-1_f64 * t63290 + 0.59266666666666666668e-1_f64 * t63293 + 0.29633333333333333334e-1_f64 * t63299 + 0.19755555555555555556e0_f64 * t63304 - 0.35560000000000000001e0_f64 * t63308 + t42078 + 0.9877777777777777778e-2_f64 * t51967;
    (t64907, t64912, t64916, t64945)
}
