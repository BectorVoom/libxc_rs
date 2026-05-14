//! MGGA_C_REVTPSS lxc pol — lxc_pol part 53 (v4rho2sigma2_8) CSE chunk 778/1089 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part53_v4rho2sigma2_8_chunk778<F: Float>(t112: F, t239: F, t624: F, t655: F, t665: F, t2339: F, t68: F, t555: F, t7063: F) -> (F, F, F, F, F) {
    let t25821 = t239 * t112;
    let t25822 = 11.0 / 9.0 * t25821;
    let t25823 = t624 * t655;
    let t25824 = t25823 * t665;
    let t25826 = t68 * t2339;
    let t25875 = t7063 * t555;
    (t25822, t25823, t25824, t25826, t25875)
}
