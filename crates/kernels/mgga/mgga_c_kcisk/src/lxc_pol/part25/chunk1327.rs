//! MGGA_C_KCISK lxc pol — lxc_pol part 25 (v4rho3sigma_5) CSE chunk 1327/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part25_v4rho3sigma_5_chunk1327<F: Float>(t34275: F, t9660: F, t1333: F, t34170: F, t34177: F, t34054: F, t34161: F, t4811: F, t34222: F, t34233: F, t1799: F, t32903: F, t6981: F, t112696: F, t116790: F, t116983: F, t17010: F, t2785: F, t34232: F, t4830: F, t9656: F, t9667: F) -> (F, F, F, F, F, F, F) {
    let t117192 = 0.18518518518518518519e-1 * t34275 * t9660;
    let t117193 = t1333 * t34170;
    let t117194 = 0.22109259259259259258e-2 * t117193;
    let t117195 = t1333 * t34177;
    let t117203 = t1333 * t34054;
    let t117204 = 0.33163888888888888888e-2 * t117203;
    let t117205 = t4811 * t34161;
    let t117206 = 0.66327777777777777776e-2 * t117205;
    let t117207 = t4811 * t34222;
    let t117211 = 0.69444444444444444446e-2 * t34233 * t9660;
    let t117213 = t1799 * t32903 * t6981;
    let t117217 = -0.69444444444444444446e-2 * t116983 * t9667 + t117192 + t117194 + 0.22109259259259259258e-2 * t117195 - 0.20833333333333333334e-1 * t17010 * t9656 * t2785 - 0.20833333333333333334e-1 * t4830 * t34232 * t2785 + t117204 + t117206 - 0.22109259259259259258e-2 * t117207 + 0.22109259259259259258e-2 * t112696 - t117211 + 0.22109259259259259258e-2 * t117213 - 0.69444444444444444446e-2 * t116790 * t9667;
    (t117193, t117195, t117203, t117205, t117207, t117213, t117217)
}
