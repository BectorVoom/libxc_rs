//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 18 (v4rho3sigma_6) CSE chunk 1005/1210 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part18_v4rho3sigma_6_chunk1005<F: Float>(t14058: F, t935: F, t4021: F, t885: F, t2149: F, t3065: F, t876: F, t1189: F, t2334: F, t2153: F, t899: F, t922: F) -> (F, F, F, F, F, F) {
    let t14059 = t14058 * t935;
    let t14063 = t4021 * t885;
    let t14064 = t14063 * t2149;
    let t14069 = t3065 * t876;
    let t14072 = t1189 * t2334;
    let t14073 = 119.0 / 6912.0 * t14072;
    let t14079 = t899 * t2153 * t922;
    (t14059, t14063, t14064, t14069, t14073, t14079)
}
