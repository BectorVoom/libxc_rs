//! MGGA_C_KCISK lxc pol — lxc_pol part 23 (v4rho3sigma_3) CSE chunk 1418/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part23_v4rho3sigma_3_chunk1418<F: Float>(t113920: F, t113922: F, t33808: F, t9532: F, t109636: F, t109643: F, t109649: F, t109652: F, t109655: F, t109662: F, t109664: F, t109669: F, t115094: F, t32436: F, t32439: F, t32485: F, t33784: F, t33823: F, t9524: F, t9851: F) -> (F,) {
    let t115374 = 0.30952962962962962962e-2 * t113920;
    let t115375 = 0.25794135802469135802e-2 * t113922;
    let t115384 = 0.34722222222222222222e-2 * t33808 * t9532;
    let t115389 = -0.20833333333333333334e-1 * t32436 * t33784 - 0.120625e-1 * t109664 * t33784 - 0.15476481481481481481e-2 * t109636 - 0.92592592592592592592e-2 * t109643 - t115374 + t115375 - 0.92592592592592592592e-2 * t109649 - 0.10416666666666666667e-1 * t9851 * t32485 + 0.10416666666666666667e-1 * t9524 * t33823 - 0.23148148148148148148e-2 * t109652 - 0.23148148148148148148e-2 * t109655 - t115384 + 0.11574074074074074074e-2 * t109662 - 0.60312500000000000001e-2 * t32439 * t115094 - 0.89351851851851851851e-3 * t109669;
    (t115389,)
}
