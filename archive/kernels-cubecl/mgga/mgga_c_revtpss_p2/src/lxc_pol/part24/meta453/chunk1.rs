//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1420/1850 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1420<F: Float>(t40424: F, t4430: F, t1561: F, t40360: F, t4372: F, t9784: F, t10504: F, t15002: F, t9285: F, t11015: F, t4325: F, t4477: F, t9292: F) -> (F, F, F, F, F, F) {
    let t51100 = t40424 * t4430;
    let t51104 = t40360 * t1561;
    let t51170 = t9784 * t4372;
    let t51203 = t10504 * t15002 * t9285;
    let t51211 = t4325 * t11015;
    let t51213 = t9292 * t4477;
    (t51100, t51104, t51170, t51203, t51211, t51213)
}
