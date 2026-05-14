//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 1049/1210 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk1049<F: Float>(t3748: F, t3975: F, t3972: F, t13544: F, t13776: F, t12213: F, t2409: F, t4164: F, t3744: F, t3959: F, t3809: F, t1178: F, t371: F, t3896: F, t3983: F, t1192: F, t3703: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t15186 = t3975 * t3748;
    let t15187 = t3972 * t15186;
    let t15191 = t3975 * t13544;
    let t15192 = t13776 * t15191;
    let t15195 = t2409 * t12213 * t4164;
    let t15198 = t3959 * t3744;
    let t15200 = t3975 * t3809;
    let t15201 = t3972 * t15200;
    let t15204 = t371 * t1178 * t3896;
    let t15205 = t3983 * t15204;
    let t15207 = t1192 * t3703;
    (t15186, t15187, t15191, t15192, t15195, t15198, t15200, t15201, t15204, t15205, t15207)
}
