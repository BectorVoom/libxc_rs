//! MGGA_C_REVTPSS lxc pol — lxc_pol part 56 (v4rho2sigma2_11) CSE chunk 636/1203 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part56_v4rho2sigma2_11_chunk636<F: Float>(t2242: F, t38: F, t644: F, t84: F, t77: F, t603: F, t607: F, t624: F, t640: F, t76: F, t1937: F, t2322: F) -> (F, F, F, F, F, F) {
    let t6954 = t2242 * t38;
    let t6959 = t84 * t644;
    let t6960 = t77 * t6959;
    let t6963 = t603 * t607;
    let t6971 = F::cast_from(8.0_f64) / F::cast_from(3.0_f64) * t624;
    let t6977 = t76 * t640;
    let t6990 = F::cast_from(2.0_f64) * t2322 * t1937;
    (t6954, t6960, t6963, t6971, t6977, t6990)
}
