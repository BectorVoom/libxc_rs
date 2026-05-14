//! GGA_C_ACGGAP lxc pol — lxc_pol part 16 (v4rho3sigma_8) CSE chunk 355/1080 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part16_v4rho3sigma_8_chunk355<F: Float>(t43: F, t50: F, t1690: F, t1694: F, t47: F, t886: F, t478: F, t52: F, t893: F, t59: F, zeta_threshold: F) -> (F, F, F) {
    let t44 = t43 <= zeta_threshold;
    let t51 = t50 <= zeta_threshold;
    let t1698 = piecewise3(t44, 0.0, 4.0 / 9.0 * t886 * t1690 + 4.0 / 3.0 * t47 * t1694);
    let t1699 = t478 * t478;
    let t1702 = -t1694;
    let t1706 = piecewise3(t51, 0.0, 4.0 / 9.0 * t893 * t1699 + 4.0 / 3.0 * t52 * t1702);
    let t1708 = (t1698 + t1706) * t59;
    (t1699, t1702, t1708)
}
