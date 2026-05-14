//! MGGA_C_REVTPSS lxc pol — lxc_pol part 34 (v4rho3sigma_9) CSE chunk 1006/1196 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part34_v4rho3sigma_9_chunk1006<F: Float>(t11239: F, t378: F, t1078: F, t1982: F, t1976: F, t3143: F, t11199: F, t1981: F, t7143: F, t11108: F, t1989: F, t2411: F, t33: F, t112: F, t239: F, t624: F, t655: F) -> (F, F, F, F, F, F, F, F) {
    let t25669 = t378 * t11239;
    let t25671 = t1982 * t25669 * t1078;
    let t25672 = t3143 * t1976;
    let t25698 = t1981 * t11199;
    let t25699 = t25698 * t7143;
    let t25713 = t1989 * t11108;
    let t25759 = t2411 * t33;
    let t25821 = t239 * t112;
    let t25822 = 11.0 / 9.0 * t25821;
    let t25823 = t624 * t655;
    (t25671, t25672, t25698, t25699, t25713, t25759, t25822, t25823)
}
