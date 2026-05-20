//! MGGA_C_REVTPSS lxc pol — lxc_pol part 33 (v4rho3sigma_8) CSE chunk 2019/2275 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2019<F: Float>(t25953: F, t26072: F, t2435: F, t25913: F, t7289: F, t94600: F, t2028: F, t3999: F, t25875: F, t25894: F, t25877: F, t94382: F) -> (F, F, F, F, F, F) {
    let t94756 = t26072 * t25953;
    let t94758 = t2435 * t25913;
    let t94761 = F::cast_from(0.39982213492741449076e-1_f64) * t7289 * t94600;
    let t94762 = t2028 * t3999;
    let t94763 = t25875 * t94762;
    let t94768 = t25894 * t94762;
    let t94771 = t94382 * t25877;
    (t94756, t94758, t94761, t94763, t94768, t94771)
}
