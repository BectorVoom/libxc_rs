//! MGGA_C_KCISK lxc pol — lxc_pol part 26 (v4rho3sigma_6) CSE chunk 1378/1407 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part26_v4rho3sigma_6_chunk1378<F: Float>(t35007: F, t3973: F, t9536: F, t109627: F, t2075: F, t33835: F, t109633: F, t115111: F, t115337: F, t115346: F, t115351: F, t115374: F, t115558: F, t115697: F, t119027: F, t1312: F, t25313: F, t32354: F, t33762: F, t33794: F, t34990: F, t35008: F, t9524: F, t9537: F, t9864: F) -> (F, F) {
    let t120257 = t9536 * t3973 * t35007;
    let t120264 = t109627 * t2075 * t33835;
    let t120279 = 0.52083333333333333333e-2 * t9524 * t34990 - 0.46429444444444444444e-2 * t119027 - 0.5787037037037037037e-3 * t120257 + 0.77160493827160493827e-3 * t115337 - 0.23148148148148148148e-2 * t115346 + t115351 - 0.34722222222222222222e-2 * t115111 * t9864 - 0.13402777777777777778e-2 * t109633 * t120264 - 0.10416666666666666667e-1 * t33794 * t33762 - 0.17361111111111111111e-2 * t32354 * t35008 - 0.17361111111111111111e-2 * t9536 * t1312 * t9537 * t25313 + 0.92592592592592592592e-2 * t115558 * t9864 + 0.92592592592592592592e-2 * t115697 * t9864 - t115374;
    (t120264, t120279)
}
