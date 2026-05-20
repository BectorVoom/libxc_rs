//! MGGA_C_REVTPSS lxc pol — lxc_pol part 40 (v4rho3tau_3) CSE chunk 1397/1507 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1397<F: Float>(t1737: F, t3476: F, t16868: F, t16712: F, t12297: F, t12299: F, t12301: F, t12303: F, t16706: F, t16727: F, t16748: F, t16871: F, t16876: F) -> (F, F) {
    let t17032 = t1737 * t3476;
    let t17050 = F::cast_from(0.13892666666666666667e0_f64) * t16868;
    let t17052 = F::cast_from(0.34431666666666666666e0_f64) * t16712;
    let t17061 = -t17050 + F::new(0.104195e0) * t16871 - t17052 + F::new(0.516475e0) * t16748 + F::cast_from(0.22954444444444444444e0_f64) * t16706 + F::cast_from(0.11577222222222222222e0_f64) * t16876 + F::cast_from(0.11477222222222222222e0_f64) * t12299 + F::cast_from(0.45908888888888888888e0_f64) * t12297 - F::cast_from(0.34431666666666666666e0_f64) * t12301 - F::cast_from(0.17215833333333333333e0_f64) * t12303 - F::cast_from(0.68863333333333333334e0_f64) * t16727;
    (t17032, t17061)
}
