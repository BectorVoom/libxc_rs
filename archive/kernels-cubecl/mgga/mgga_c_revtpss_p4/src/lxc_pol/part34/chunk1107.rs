//! MGGA_C_REVTPSS lxc pol — lxc_pol part 34 (v4rho3sigma_9) CSE chunk 1107/1341 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part34_v4rho3sigma_9_chunk1107<F: Float>(t25698: F, t7143: F, t11108: F, t1989: F, t2411: F, t33: F, t112: F, t239: F, t624: F, t655: F, t2339: F, t68: F) -> (F, F, F, F, F, F) {
    let t25699 = t25698 * t7143;
    let t25713 = t1989 * t11108;
    let t25759 = t2411 * t33;
    let t25821 = t239 * t112;
    let t25822 = F::cast_from(11.0_f64) / F::cast_from(9.0_f64) * t25821;
    let t25823 = t624 * t655;
    let t25826 = t68 * t2339;
    (t25699, t25713, t25759, t25822, t25823, t25826)
}
