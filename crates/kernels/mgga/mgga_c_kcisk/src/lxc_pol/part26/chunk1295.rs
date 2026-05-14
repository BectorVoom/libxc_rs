//! MGGA_C_KCISK lxc pol — lxc_pol part 26 (v4rho3sigma_6) CSE chunk 1295/1407 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part26_v4rho3sigma_6_chunk1295<F: Float>(t33870: F, t9524: F, t2737: F, t33863: F, t4419: F, t32339: F, t33883: F, t32354: F, t114995: F, t32439: F, t123: F, t2734: F, t33849: F, t114437: F, t114439: F, t114453: F) -> (F, F, F, F, F, F, F, F, F) {
    let t115676 = 0.34722222222222222222e-2 * t9524 * t33870;
    let t115679 = 0.34722222222222222222e-2 * t2737 * t4419 * t33863;
    let t115684 = t32339 * t33883;
    let t115693 = 0.11574074074074074074e-2 * t32354 * t33883;
    let t115695 = 0.13402777777777777778e-2 * t32439 * t114995;
    let t115697 = t2734 * t33849 * t123;
    let t115704 = 0.23214722222222222222e-2 * t114437;
    let t115705 = 0.10317654320987654321e-2 * t114439;
    let t115708 = 0.15476481481481481481e-2 * t114453;
    (t115676, t115679, t115684, t115693, t115695, t115697, t115704, t115705, t115708)
}
