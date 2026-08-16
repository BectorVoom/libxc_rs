//! MGGA_C_REVTPSS lxc pol — lxc_pol part 30 (v4rho3sigma_5) CSE chunk 2040/2270 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2040<F: Float>(t545: F, t94667: F, t25875: F, t25925: F, t686: F, t72: F, t25894: F, t25950: F, t25953: F, t26069: F, t94407: F, t1445: F, t25912: F, t689: F) -> (F, F, F, F, F, F, F) {
    let t94668 = t94667 * t545;
    let t94669 = t25875 * t94668;
    let t94671 = t25925 * t72 * t686;
    let t94672 = t94669 * t94671;
    let t94674 = t25894 * t94668;
    let t94675 = t94674 * t94671;
    let t94677 = t25950 * t25953;
    let t94682 = F::cast_from(0.91399340044406952588e-2_f64) * t26069 * t94407;
    let t94694 = t689 * t25912 * t1445;
    (t94669, t94672, t94674, t94675, t94677, t94682, t94694)
}
