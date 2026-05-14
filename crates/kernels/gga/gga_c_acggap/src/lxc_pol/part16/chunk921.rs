//! GGA_C_ACGGAP lxc pol — lxc_pol part 16 (v4rho3sigma_8) CSE chunk 921/1080 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part16_v4rho3sigma_8_chunk921<F: Float>(t2035: F, t36213: F, t7323: F, t142: F, t3706: F, t30248: F, t532: F, t537: F, t7637: F, t8859: F, t1576: F, t7614: F, t1181: F, t5249: F, t599: F, t7493: F) -> (F, F, F, F, F, F, F) {
    let t36214 = t2035 * t7323 * t36213;
    let t36215 = 0.916875e-1 * t36214;
    let t36222 = t142 * t3706;
    let t36231 = t30248 * t532;
    let t36236 = t30248 * t537;
    let t36238 = t7637 * t8859;
    let t36240 = t7614 * t1576;
    let t36273 = t7493 * t1181 * t599 * t5249;
    (t36215, t36222, t36231, t36236, t36238, t36240, t36273)
}
