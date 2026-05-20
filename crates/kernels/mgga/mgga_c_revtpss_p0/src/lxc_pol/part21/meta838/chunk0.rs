//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 3139/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3139<F: Float>(t1196: F, t12548: F, t5197: F, t16643: F, t3531: F, t16682: F, t1732: F, t3433: F, t12411: F, t12556: F, t1756: F, t43752: F) -> (F, F, F, F, F) {
    let t57849 = F::cast_from(0.11696447245269292414e1_f64) * t1196 * t5197 * t12548;
    let t57851 = F::cast_from(0.31168546390226634765e3_f64) * t3531 * t16643;
    let t57853 = F::cast_from(0.35089341735807877242e1_f64) * t3531 * t16682;
    let t57854 = t3433 * t1732;
    let t57856 = F::new(18.0) * t57854 * t12411;
    let t57860 = F::cast_from(0.12304822629859687989e5_f64) * t1196 * t43752 * t1756 * t12556;
    (t57849, t57851, t57853, t57856, t57860)
}
