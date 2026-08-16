//! MGGA_C_REVTPSS lxc pol — lxc_pol part 56 (v4rho2sigma2_11) CSE chunk 955/1203 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part56_v4rho2sigma2_11_chunk955<F: Float>(t117: F, t33374: F, t32172: F, t32174: F, t32176: F, t32178: F, t32828: F, t32830: F, t32832: F, t33346: F, t670: F, t8564: F) -> (F, F) {
    let t33375 = t33374 * t117;
    let t33381 = F::cast_from(2.0_f64) * t33346 * t670 + t32172 + t32174 + t32176 + t32178 + F::cast_from(4.0_f64) * t32828 + F::cast_from(4.0_f64) * t32830 + F::cast_from(4.0_f64) * t32832 + t33375 + t8564;
    (t33375, t33381)
}
