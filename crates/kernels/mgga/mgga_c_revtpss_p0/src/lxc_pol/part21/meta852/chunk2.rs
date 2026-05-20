//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 3204/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3204<F: Float>(t17500: F, t372: F, t13142: F, t56878: F, t12866: F, t17514: F, t56756: F, t12916: F, t17723: F, t3718: F, t13043: F, t1774: F) -> (F, F, F, F, F) {
    let t59062 = t372 * t17500;
    let t59066 = t13142 * t56878;
    let t59078 = t12866 * t56756 * t17514;
    let t59094 = t3718 * t12916 * t17723;
    let t59096 = t1774 * t13043;
    (t59062, t59066, t59078, t59094, t59096)
}
