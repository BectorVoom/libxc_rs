//! MGGA_C_REVTPSS lxc pol — lxc_pol part 33 (v4rho3sigma_8) CSE chunk 1780/2275 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1780<F: Float>(t112: F, t239: F, t624: F, t655: F, t665: F, t2339: F, t68: F, t2033: F, t530: F, t555: F, t7063: F) -> (F, F, F, F, F, F) {
    let t25821 = t239 * t112;
    let t25822 = F::cast_from(11.0_f64) / F::cast_from(9.0_f64) * t25821;
    let t25823 = t624 * t655;
    let t25824 = t25823 * t665;
    let t25826 = t68 * t2339;
    let t25864 = t530 * t2033;
    let t25875 = t7063 * t555;
    (t25822, t25823, t25824, t25826, t25864, t25875)
}
