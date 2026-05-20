//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 3009/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3009<F: Float>(t1011: F, t140: F, t23868: F, t41361: F, t42078: F, t51978: F, t53243: F, t63276: F, t63278: F, t77499: F, t77503: F, t77505: F, t77507: F, t77509: F, t77515: F, t77518: F, t77521: F, t77527: F, t77531: F, t77535: F, t77539: F, t77543: F, t77547: F) -> (F, F) {
    let t79957 = t1011 * t140 * t23868;
    let t80008 = F::cast_from(0.5487654320987654321e-2_f64) * t77499 - F::cast_from(0.14816666666666666667e-1_f64) * t77503 + F::cast_from(0.4938888888888888889e-2_f64) * t77505 - F::cast_from(0.19755555555555555556e-1_f64) * t77507 + F::cast_from(0.29633333333333333334e-1_f64) * t77509 - F::cast_from(0.29633333333333333334e-1_f64) * t63276 + F::cast_from(0.9877777777777777778e-2_f64) * t63278 + t42078 + F::cast_from(0.17780000000000000001e0_f64) * t77515 - F::cast_from(0.4938888888888888889e-1_f64) * t77518 - F::cast_from(0.26670000000000000001e0_f64) * t77521 - t53243 + F::cast_from(0.46096296296296296297e-1_f64) * t51978 + F::cast_from(0.15365432098765432099e-1_f64) * t41361 - F::cast_from(0.29633333333333333334e-1_f64) * t77527 - F::cast_from(0.29633333333333333334e-1_f64) * t77531 + F::cast_from(0.35560000000000000001e0_f64) * t77535 - F::cast_from(0.26670000000000000001e0_f64) * t77539 + F::cast_from(0.88900000000000000002e-1_f64) * t77543 + F::cast_from(0.88900000000000000002e-1_f64) * t77547;
    (t79957, t80008)
}
