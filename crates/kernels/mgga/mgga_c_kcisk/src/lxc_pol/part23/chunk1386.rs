//! MGGA_C_KCISK lxc pol — lxc_pol part 23 (v4rho3sigma_3) CSE chunk 1386/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part23_v4rho3sigma_3_chunk1386<F: Float>(t1308: F, t14242: F, t2158: F, t109956: F, t110725: F, t113740: F, t114059: F, t114552: F, t114555: F, t114558: F, t114566: F, t114573: F, t114577: F, t13448: F, t2718: F, t32022: F, t32087: F, t33389: F, t33520: F, t3491: F, t9429: F, t9454: F, t9800: F) -> (F,) {
    let t114580 = t14242 * t2158 * t1308;
    let t114583 = -0.55555555555555555558e-1 * t114059 * t9454 - 0.55555555555555555558e-1 * t114059 * t9429 + 0.22109259259259259258e-2 * t109956 - 0.69444444444444444446e-2 * t32087 * t113740 + 0.66327777777777777776e-2 * t114552 + 0.13265555555555555555e-1 * t114555 - 0.13265555555555555555e-1 * t114558 - 0.20833333333333333334e-1 * t3491 * t33520 * t2718 + 0.11111111111111111112e0 * t32022 * t33389 - 0.66327777777777777776e-2 * t114566 - 0.23148148148148148148e-2 * t110725 - 0.10416666666666666667e-1 * t13448 * t9800 * t2718 + 0.16581944444444444444e-2 * t114573 - 0.16581944444444444444e-2 * t114577 + 0.8041666666666666667e-2 * t114580 * t9429;
    (t114583,)
}
