//! MGGA_C_KCISK lxc pol — lxc_pol part 23 (v4rho3sigma_3) CSE chunk 1108/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part23_v4rho3sigma_3_chunk1108<F: Float>(t2709: F, t31877: F, t1155: F, t1624: F, t2685: F, t3185: F, t3187: F, t3472: F, t559: F, t1156: F, t9575: F, t294: F, t113: F, t2932: F, t20: F, t446: F) -> (F, F, F, F, F, F, F, F, F) {
    let t31878 = t2709 * t31877;
    let t31879 = t31878 / 16.0;
    let t31880 = t1155 * t1624;
    let t31881 = t2709 * t31880;
    let t31882 = t31881 / 8.0;
    let t31883 = t2685 * t3185;
    let t31884 = t31883 * t3187;
    let t31885 = 2.0 * t31884;
    let t31886 = t3472 * t559;
    let t31887 = t2709 * t31886;
    let t31888 = t31887 / 16.0;
    let t31890 = t1156 * t9575;
    let t31891 = t294 * t31890;
    let t31892 = t31891 / 8.0;
    let t31893 = t2932 * t113;
    let t31894 = t446 * t20;
    (t31879, t31882, t31883, t31884, t31885, t31888, t31892, t31893, t31894)
}
