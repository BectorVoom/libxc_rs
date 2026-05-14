//! MGGA_C_KCISK lxc pol — lxc_pol part 25 (v4rho3sigma_5) CSE chunk 1326/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part25_v4rho3sigma_5_chunk1326<F: Float>(t116477: F, t33002: F, t34228: F, t9660: F, t2508: F, t5060: F, t415: F, t5064: F, t34264: F, t654: F, t719: F, t17032: F, t5182: F, t112266: F, t112665: F, t112667: F, t112669: F, t112674: F, t112683: F, t116916: F, t1693: F, t17705: F, t20: F, t2785: F, t32917: F, t33031: F, t34018: F, t34125: F) -> (F, F, F) {
    let t117159 = 0.15520416666666666667e-2 * t33002 * t116477;
    let t117161 = 0.69444444444444444446e-2 * t34228 * t9660;
    let t117165 = t2508 * t5060;
    let t117167 = t415 * t117165 * t5064;
    let t117170 = 0.69444444444444444446e-2 * t34264 * t9660;
    let t117182 = t5060 * t654 * t719;
    let t117184 = t5182 * t117182 * t17032;
    let t117187 = -0.18518518518518518519e-1 * t34125 * t32917 - t117159 - t117161 + 0.92592592592592592594e-2 * t33031 * t116916 + 0.16581944444444444444e-2 * t112665 + 0.49745833333333333332e-2 * t117167 - t117170 - 0.69444444444444444446e-2 * t112667 - 0.34722222222222222223e-2 * t112669 - 0.92592592592592592594e-2 * t112266 * t34018 - 0.10416666666666666667e-1 * t1693 * t17705 * t654 * t20 * t2785 - 0.22109259259259259258e-2 * t112674 + 0.66327777777777777776e-2 * t117184 - 0.16581944444444444444e-2 * t112683;
    (t117167, t117184, t117187)
}
