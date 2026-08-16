//! MGGA_C_REVTPSS lxc pol — lxc_pol part 32 (v4rho3sigma_7) CSE chunk 1852/2056 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1852<F: Float>(t25375: F, t95628: F, t136: F, t137: F, t2061: F, t10505: F, t93377: F, t7406: F, t9288: F, t7064: F, t10073: F, t25308: F, t26554: F) -> (F, F, F, F, F, F, F) {
    let t95722 = t25375 * t95628;
    let t95725 = t2061 * t136 * t137;
    let t95726 = t95725 * t10505;
    let t95727 = t93377 * t95726;
    let t95730 = t7406 * t9288;
    let t95732 = F::cast_from(0.39982213492741449076e-1_f64) * t7064 * t95730;
    let t95740 = t10073 * t25308 * t26554;
    (t95722, t95725, t95726, t95727, t95730, t95732, t95740)
}
