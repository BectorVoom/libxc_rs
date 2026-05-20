//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 2018/2259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2018<F: Float>(t25895: F, t94661: F, t1032: F, t9656: F, t545: F, t25875: F, t25894: F, t25950: F, t25953: F, t26069: F, t94407: F, t7282: F, t9646: F) -> (F, F, F, F, F, F) {
    let t94662 = t25895 * t94661;
    let t94667 = t1032 * t9656;
    let t94668 = t94667 * t545;
    let t94669 = t25875 * t94668;
    let t94674 = t25894 * t94668;
    let t94677 = t25950 * t25953;
    let t94682 = F::cast_from(0.91399340044406952588e-2_f64) * t26069 * t94407;
    let t94696 = t9646 * t7282;
    (t94662, t94669, t94674, t94677, t94682, t94696)
}
