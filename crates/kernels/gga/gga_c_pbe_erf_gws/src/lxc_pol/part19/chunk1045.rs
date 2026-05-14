//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 1045/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk1045<F: Float>(t12109: F, t2409: F, t3965: F, t1161: F, t343: F, t14724: F, t13796: F, t3989: F, t1178: F, t371: F, t3887: F, t1177: F, t1118: F, t1134: F, t13859: F, t3748: F, t3975: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t15164 = t2409 * t12109;
    let t15165 = t3965 * t15164;
    let t15167 = t343 * t1161;
    let t15168 = t14724 * t15167;
    let t15169 = t13796 * t15168;
    let t15170 = t3989 * t15169;
    let t15177 = t371 * t1178 * t3887;
    let t15178 = t1177 * t15177;
    let t15181 = t1118 * t1134;
    let t15182 = t13796 * t15181;
    let t15183 = t13859 * t15182;
    let t15186 = t3975 * t3748;
    (t15164, t15165, t15167, t15169, t15170, t15177, t15178, t15182, t15183, t15186)
}
