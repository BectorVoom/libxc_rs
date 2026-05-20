//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2949/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2949<F: Float>(t11466: F, t11507: F, t11554: F, t15413: F, t1634: F, t19021: F, t19294: F, t19297: F, t23711: F, t23761: F, t23785: F, t2987: F, t3012: F, t4707: F, t4708: F, t52443: F, t6190: F, t6205: F, t78303: F, t78305: F, t78307: F, t78309: F, t78311: F, t78313: F, t78315: F, t972: F) -> F {
    let t78316 = -F::cast_from(0.70178683471615754484e1_f64) * t15413 * t19294 - F::cast_from(0.31168546390226634765e3_f64) * t52443 * t19297 - F::cast_from(0.14035736694323150897e2_f64) * t11466 * t23711 * t972 + F::cast_from(0.10526802520742363173e2_f64) * t3012 * t6190 * t4707 + F::cast_from(0.6233709278045326953e3_f64) * t11507 * t23785 * t972 - F::cast_from(0.35089341735807877242e1_f64) * t11554 * t23761 - F::cast_from(0.35089341735807877242e1_f64) * t2987 * t4708 * t6205 - F::cast_from(0.35089341735807877242e1_f64) * t2987 * t1634 * t19021 + t78303 - t78305 + t78307 - t78309 + t78311 - t78313 - t78315;
    t78316
}
