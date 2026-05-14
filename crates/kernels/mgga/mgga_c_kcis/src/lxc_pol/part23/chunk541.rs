//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 541/1177 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk541<F: Float>(t174: F, t4879: F, t7: F, t118: F, t251: F, t66: F, t148: F, t41: F, t85: F, t4532: F, t447: F, t2002: F, t3734: F, t1464: F, t1988: F, t3255: F, t3752: F, t544: F, zeta_threshold: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t175 = t174 <= zeta_threshold;
    let t4880 = t7 * t4879;
    let t4881 = t118 * t4880;
    let t4992 = t66 * t251;
    let t5133 = t85 * t148 * t41;
    let t5406 = piecewise3(t175, 0.0, -t4532);
    let t5407 = t447 * t5406;
    let t5417 = t3734 * t2002;
    let t5418 = t1464 * t5417;
    let t5423 = t3255 * t1988;
    let t5425 = t3752 * t544;
    (t4880, t4881, t4992, t5133, t5406, t5407, t5417, t5418, t5423, t5425)
}
