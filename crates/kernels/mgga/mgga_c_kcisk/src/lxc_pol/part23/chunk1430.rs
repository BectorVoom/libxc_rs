//! MGGA_C_KCISK lxc pol — lxc_pol part 23 (v4rho3sigma_3) CSE chunk 1430/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part23_v4rho3sigma_3_chunk1430<F: Float>(t32354: F, t33883: F, t114995: F, t32439: F, t123: F, t2734: F, t33849: F, t114437: F, t114439: F, t114453: F, t4350: F, t539: F, t6174: F, t32466: F, t5670: F, t109633: F, t109880: F, t109891: F, t109919: F, t114444: F, t114448: F, t114455: F, t114458: F, t32447: F, t33794: F, t9539: F) -> (F, F, F) {
    let t115693 = 0.11574074074074074074e-2 * t32354 * t33883;
    let t115695 = 0.13402777777777777778e-2 * t32439 * t114995;
    let t115697 = t2734 * t33849 * t123;
    let t115704 = 0.23214722222222222222e-2 * t114437;
    let t115705 = 0.10317654320987654321e-2 * t114439;
    let t115708 = 0.15476481481481481481e-2 * t114453;
    let t115710 = t539 * t4350;
    let t115711 = t6174 * t115710;
    let t115713 = t115711 * t5670 * t32466;
    let t115718 = -t115693 - t115695 + 0.92592592592592592593e-2 * t115697 * t9539 + 0.34722222222222222222e-2 * t33794 * t32447 - 0.77160493827160493826e-3 * t109880 + 0.11607361111111111111e-2 * t109891 - t115704 + t115705 + 0.23214722222222222222e-2 * t114444 + 0.61905925925925925926e-2 * t114448 + t115708 - 0.38691203703703703704e-2 * t114455 + 0.1787037037037037037e-2 * t109633 * t115713 - 0.17411041666666666666e-2 * t114458 - 0.41270617283950617284e-2 * t109919;
    (t115710, t115713, t115718)
}
