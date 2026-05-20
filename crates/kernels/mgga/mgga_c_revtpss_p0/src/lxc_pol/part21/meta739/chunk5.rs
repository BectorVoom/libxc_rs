//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2597/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2597<F: Float>(t13746: F, t686: F, t72: F, t9680: F, t14085: F, t2435: F, t14104: F, t47520: F, t10069: F, t13731: F, t137: F, t14103: F, t47480: F, t9675: F) -> (F, F, F, F, F) {
    let t47832 = t9680 * t13746 * t72 * t686;
    let t47834 = t2435 * t14085;
    let t47835 = F::cast_from(0.21951497276451705329e-1_f64) * t47834;
    let t47837 = t47520 * t14104;
    let t47838 = F::cast_from(0.34697458558045176417e-2_f64) * t47837;
    let t47839 = t10069 * t13731;
    let t47844 = t47480 * t14103 * t137 * t9675;
    (t47832, t47835, t47838, t47839, t47844)
}
