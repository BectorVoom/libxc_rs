//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 1251/1404 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk1251<F: Float>(t14843: F, t804: F, t15097: F, t2053: F, t15081: F, t2376: F, t829: F, t830: F, t52901: F, t52930: F, t52961: F, t52968: F) -> (F, F, F, F, F, F, F) {
    let t54866 = F::cast_from(6.0_f64) * t804 * t14843;
    let t54867 = t15097 * t2053;
    let t54880 = t2376 * t15081;
    let t54882 = t829 * t830 * t54880;
    let t54886 = F::cast_from(7.0_f64) / F::cast_from(576.0_f64) * t52901;
    let t54896 = F::cast_from(7.0_f64) / F::cast_from(72.0_f64) * t52930;
    let t54902 = F::cast_from(7.0_f64) / F::cast_from(1152.0_f64) * t52961;
    let t54904 = F::cast_from(7.0_f64) / F::cast_from(576.0_f64) * t52968;
    (t54866, t54867, t54882, t54886, t54896, t54902, t54904)
}
