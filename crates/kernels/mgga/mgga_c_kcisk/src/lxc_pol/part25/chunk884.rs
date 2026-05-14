//! MGGA_C_KCISK lxc pol — lxc_pol part 25 (v4rho3sigma_5) CSE chunk 884/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part25_v4rho3sigma_5_chunk884<F: Float>(t15909: F, t6666: F, t5192: F, t15903: F, t15845: F, t15849: F, t15853: F, t15856: F, t15860: F, t15864: F, t15866: F, t15871: F, t15876: F, t15878: F, t15882: F, t15884: F, t15886: F, t15888: F, t15895: F, t15900: F, t15907: F) -> (F, F, F) {
    let t15910 = t6666 * t15909;
    let t15911 = t5192 * t15910;
    let t15912 = t15903 * t15911;
    let t15914 = 0.55273148148148148146e-2 * t15845 + 0.33163888888888888888e-2 * t15849 + 0.33163888888888888888e-2 * t15853 + 0.16581944444444444444e-2 * t15856 + 0.88437037037037037034e-2 * t15860 - 0.5895802469135802469e-2 * t15864 - 0.5895802469135802469e-2 * t15866 + 0.55273148148148148147e-2 * t15871 - 0.16581944444444444444e-2 * t15876 + 0.22109259259259259258e-2 * t15878 + 0.11054629629629629629e-2 * t15882 - 0.33163888888888888888e-2 * t15884 + 0.22109259259259259258e-2 * t15886 - 0.66327777777777777776e-2 * t15888 + 0.66327777777777777776e-2 * t15895 - 0.44218518518518518517e-2 * t15900 - 0.44218518518518518517e-2 * t15907 + 0.13265555555555555555e-1 * t15912;
    (t15910, t15912, t15914)
}
