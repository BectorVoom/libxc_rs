//! MGGA_C_KCISK lxc pol — lxc_pol part 23 (v4rho3sigma_3) CSE chunk 989/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part23_v4rho3sigma_3_chunk989<F: Float>(t1411: F, t20009: F, t4158: F, t5874: F, t1220: F, t14218: F, t14220: F, t14224: F, t14226: F, t14228: F, t14230: F, t14242: F, t19969: F, t19985: F, t19988: F, t19994: F, t19996: F, t20000: F, t20002: F, t20007: F, t3491: F, t3930: F, t4159: F, t5875: F, t6218: F, t6221: F) -> (F, F, F) {
    let t20010 = t1411 * t20009;
    let t20019 = t5874 * t4158;
    let t20023 = -0.386e0 * t1220 * t19969 + 0.11054629629629629629e-2 * t14218 - 0.33163888888888888888e-2 * t14220 + 0.24320185185185185185e-1 * t19985 - 0.49745833333333333332e-2 * t19988 - 0.193e0 * t6221 * t4159 - 0.386e0 * t3491 * t6218 + 0.22109259259259259258e-2 * t19994 - 0.33163888888888888888e-2 * t19996 - 0.33163888888888888888e-2 * t20000 - 0.22109259259259259258e-2 * t20002 - 0.16581944444444444444e-2 * t20007 - 0.11054629629629629629e-2 * t20010 + 0.11054629629629629629e-2 * t14224 - 0.73697530864197530861e-3 * t14226 - 0.58958024691358024689e-2 * t14228 + 0.386e0 * t3491 * t5875 + 0.148996e0 * t14242 * t5875 + 0.74498e-1 * t3930 * t20019 - 0.22109259259259259258e-2 * t14230;
    (t20010, t20019, t20023)
}
