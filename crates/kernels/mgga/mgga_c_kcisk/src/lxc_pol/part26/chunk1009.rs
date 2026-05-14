//! MGGA_C_KCISK lxc pol — lxc_pol part 26 (v4rho3sigma_6) CSE chunk 1009/1407 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part26_v4rho3sigma_6_chunk1009<F: Float>(t19055: F, t5622: F, t5628: F, t2211: F, t3529: F, t1336: F, t140: F, t5636: F, t13959: F, t8177: F, t13311: F, t8176: F, t3482: F, t1220: F, t1329: F, t14250: F, t20886: F, t20892: F, t20893: F, t20896: F, t20898: F, t25906: F, t26999: F, t27006: F, t27008: F, t27013: F, t27016: F, t3930: F, t412: F, t5875: F, t6221: F) -> (F, F, F, F, F, F) {
    let t27028 = t19055 * t5622;
    let t27030 = t19055 * t5628;
    let t27032 = t3529 * t2211;
    let t27034 = t140 * t1336 * t27032;
    let t27035 = t27034 * t5636;
    let t27037 = t13959 * t8177;
    let t27039 = t13311 * t8176;
    let t27040 = t3482 * t27039;
    let t27042 = 0.14739506172839506173e-2 * t14250 - 0.44218518518518518517e-2 * t27006 + 0.18424382716049382715e-2 * t27008 + 0.49745833333333333332e-2 * t27013 + t20892 - 0.44218518518518518516e-2 * t20893 + t20896 - t20898 - 0.193e0 * t27016 * t1329 - 0.223494e0 * t3930 * t26999 + t25906 * t412 + 0.386e0 * t6221 * t5875 + 0.148996e0 * t20886 * t5875 - 0.386e0 * t1220 * t26999 + 0.22109259259259259259e-2 * t27028 - 0.66327777777777777776e-2 * t27030 + 0.55273148148148148147e-2 * t27035 + 0.14739506172839506172e-2 * t27037 - 0.58958024691358024688e-2 * t27040;
    (t27028, t27030, t27035, t27037, t27040, t27042)
}
