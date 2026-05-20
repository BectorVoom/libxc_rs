//! MGGA_C_REVTPSS lxc pol — lxc_pol part 33 (v4rho3sigma_8) CSE chunk 2252/2275 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2252<F: Float>(t20842: F, t7613: F, t1234: F, t30815: F, t20816: F, t7618: F, t29020: F, t5265: F, t104953: F, t104963: F, t104968: F, t1238: F, t20792: F, t21085: F, t21157: F, t26867: F, t7624: F, t97267: F, t97272: F) -> F {
    let t112452 = t7613 * t20842;
    let t112456 = t1234 * t30815;
    let t112461 = t7618 * t20816;
    let t112465 = t29020 * t5265;
    let t112467 = -F::cast_from(0.28582678745379824648e-3_f64) * t26867 * t21157 - t104953 - F::cast_from(0.28582678745379824648e-3_f64) * t112452 - F::cast_from(0.42874018118069736972e-3_f64) * t7613 * t21085 - F::cast_from(0.14481890564325777821e-1_f64) * t112456 * t1238 + t104963 / F::new(81.0) - F::cast_from(0.95275595817932748827e-4_f64) * t97267 + t97272 + F::cast_from(0.28582678745379824648e-3_f64) * t112461 + F::cast_from(0.47637797908966374413e-3_f64) * t7624 * t20792 - F::cast_from(0.30488190661738479624e-2_f64) * t112465 - t104968;
    t112467
}
