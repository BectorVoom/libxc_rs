//! MGGA_C_REVTPSS lxc pol — lxc_pol part 34 (v4rho3sigma_9) CSE chunk 1211/1341 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part34_v4rho3sigma_9_chunk1211<F: Float>(t545: F, t94667: F, t25875: F, t25894: F, t26069: F, t94407: F, t1426: F, t9990: F, t7282: F, t9646: F, t2022: F, t22: F, t25937: F) -> (F, F, F, F, F, F) {
    let t94668 = t94667 * t545;
    let t94669 = t25875 * t94668;
    let t94674 = t25894 * t94668;
    let t94682 = F::cast_from(0.91399340044406952588e-2_f64) * t26069 * t94407;
    let t94683 = t1426 * t9990;
    let t94696 = t9646 * t7282;
    let t94698 = t25937 * t2022 * t22;
    (t94669, t94674, t94682, t94683, t94696, t94698)
}
