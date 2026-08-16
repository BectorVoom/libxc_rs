//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2621/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2621<F: Float>(t47005: F, t47007: F, t13597: F, t2626: F, t5571: F, t9387: F, t47009: F, t47011: F, t47013: F, t13613: F, t2619: F, t9323: F) -> (F, F, F, F, F, F, F, F, F) {
    let t48258 = F::cast_from(72.0_f64) * t47005;
    let t48259 = F::cast_from(144.0_f64) * t47007;
    let t48260 = t13597 * t2626;
    let t48261 = F::cast_from(0.35089341735807877242e1_f64) * t48260;
    let t48262 = t5571 * t9387;
    let t48263 = F::cast_from(0.5848223622634646207e0_f64) * t48262;
    let t48264 = F::cast_from(0.73245789224026180215e-3_f64) * t47009;
    let t48265 = F::cast_from(0.17090684152272775383e-2_f64) * t47011;
    let t48266 = F::cast_from(48.0_f64) * t47013;
    let t48267 = t13613 * t2619;
    let t48268 = F::cast_from(0.73245789224026180216e-3_f64) * t48267;
    let t48269 = t5571 * t9323;
    (t48258, t48259, t48261, t48263, t48264, t48265, t48266, t48268, t48269)
}
