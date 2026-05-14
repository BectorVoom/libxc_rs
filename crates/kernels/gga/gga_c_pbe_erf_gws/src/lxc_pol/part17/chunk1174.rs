//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 17 (v4rho3sigma_5) CSE chunk 1174/1178 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part17_v4rho3sigma_5_chunk1174<F: Float>(t13781: F, t13782: F, t3038: F, t3972: F, t1192: F, t26654: F, t829: F, t830: F, t13792: F, t8716: F, t13808: F, t14584: F, t4130: F, t51650: F, t2409: F, t26880: F, t3959: F) -> (F, F, F, F, F, F) {
    let t54707 = t3972 * t13781 * t3038 * t13782;
    let t54709 = t26654 * t1192;
    let t54711 = t829 * t830 * t54709;
    let t54714 = t13792 * t8716;
    let t54716 = t13808 * t14584;
    let t54717 = 7.0 / 1152.0 * t54716;
    let t54719 = t51650 * t4130;
    let t54722 = t3959 * t2409 * t26880;
    (t54707, t54711, t54714, t54717, t54719, t54722)
}
