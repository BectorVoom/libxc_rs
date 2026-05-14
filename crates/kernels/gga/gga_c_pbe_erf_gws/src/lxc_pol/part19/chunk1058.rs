//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 1058/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk1058<F: Float>(t1205: F, t3886: F, t2409: F, t3067: F, t1144: F, t338: F, t4228: F, t1109: F, t1206: F, t353: F, t859: F, t3717: F, t2376: F, t12213: F, t4216: F, t14185: F, t3742: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t15526 = t1205 * t3886;
    let t15528 = t2409 * t3067 * t15526;
    let t15532 = t338 * t1144 * t4228;
    let t15535 = t1206 * t1109;
    let t15536 = t353 * t15535;
    let t15537 = t859 * t15536;
    let t15543 = t1205 * t3717;
    let t15545 = t2409 * t2376 * t15543;
    let t15550 = t2409 * t12213 * t4216;
    let t15558 = t14185 * t3742;
    (t15526, t15528, t15532, t15535, t15536, t15537, t15543, t15545, t15550, t15558)
}
