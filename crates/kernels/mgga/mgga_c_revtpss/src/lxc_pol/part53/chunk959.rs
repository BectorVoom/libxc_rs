//! MGGA_C_REVTPSS lxc pol — lxc_pol part 53 (v4rho2sigma2_8) CSE chunk 959/1089 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part53_v4rho2sigma2_8_chunk959<F: Float>(t25081: F, t8567: F, t11064: F, t8489: F, t198: F, t206: F, t8493: F, t41154: F, t2411: F, t31858: F, t31844: F, t8478: F, t8479: F, t246: F, t826: F, t854: F) -> (F, F, F, F, F, F, F, F, F) {
    let t119578 = t8567 * t25081;
    let t119675 = t8489 * t11064;
    let t119706 = t198 * t206 * t8493;
    let t119711 = t8493 * t41154;
    let t119737 = t31858 * t2411;
    let t119747 = t198 * t206 * t8489;
    let t119751 = t8478 * t8479 * t31844;
    let t119752 = t826 * t246;
    let t119757 = t854 * t246;
    (t119578, t119675, t119706, t119711, t119737, t119747, t119751, t119752, t119757)
}
