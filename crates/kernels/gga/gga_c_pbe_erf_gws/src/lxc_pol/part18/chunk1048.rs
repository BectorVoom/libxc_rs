//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 18 (v4rho3sigma_6) CSE chunk 1048/1210 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part18_v4rho3sigma_6_chunk1048<F: Float>(t15164: F, t3965: F, t1161: F, t343: F, t14724: F, t13796: F, t3989: F, t1178: F, t371: F, t3887: F, t1177: F, t1118: F, t1134: F, t13859: F, t1115: F, t14397: F, t14400: F, t14404: F, t14420: F, t14599: F, t14898: F, t15135: F, t15139: F, t15147: F, t15152: F, t15156: F, t15162: F, t3066: F, t3917: F, t4002: F, t8629: F, t8793: F) -> (F, F, F, F, F) {
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
    let t15185 = 7.0 / 144.0 * t14400 + t8793 * t14420 / 24.0 - t15135 / 768.0 + t8629 * t15139 / 96.0 + t8793 * t14404 / 24.0 - t15147 / 768.0 - t15152 / 1536.0 + t3066 * t15156 / 48.0 + t15162 / 96.0 + t15165 / 48.0 + t15170 / 1536.0 - t3917 * t4002 / 96.0 - t1115 * t14397 / 48.0 - t15178 / 3072.0 - 7.0 / 72.0 * t14599 + t15183 / 384.0 + t14898;
    (t15167, t15169, t15177, t15182, t15185)
}
