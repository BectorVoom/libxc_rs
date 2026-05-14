//! MGGA_C_KCISK lxc pol — lxc_pol part 26 (v4rho3sigma_6) CSE chunk 1336/1407 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part26_v4rho3sigma_6_chunk1336<F: Float>(t13485: F, t32087: F, t34767: F, t119268: F, t32008: F, t1411: F, t2262: F, t33608: F, t5967: F, t109420: F, t34723: F, t2152: F, t32045: F, t51845: F, t110219: F, t110524: F, t110648: F, t113997: F, t114059: F, t114438: F, t114440: F, t114454: F, t34763: F, t34768: F, t9809: F) -> (F, F, F, F) {
    let t119419 = t32087 * t13485 * t34767;
    let t119423 = t32008 * t119268;
    let t119427 = t1411 * t33608 * t2262 * t5967;
    let t119430 = t1411 * t109420 * t34723;
    let t119438 = t1411 * t32045 * t51845 * t2152;
    let t119440 = -t114438 + t114440 + 0.7716049382716049383e-3 * t110648 - 0.18518518518518518519e-1 * t110524 * t34768 + 0.23148148148148148149e-2 * t119419 - 0.71481481481481481483e-2 * t110219 * t34763 + 0.89351851851851851853e-3 * t119423 + 0.99491666666666666664e-2 * t119427 - 0.33163888888888888888e-2 * t119430 - 0.55555555555555555557e-1 * t114059 * t9809 - 0.55555555555555555557e-1 * t113997 * t9809 - 0.33163888888888888888e-2 * t119438 + t114454;
    (t119427, t119430, t119438, t119440)
}
