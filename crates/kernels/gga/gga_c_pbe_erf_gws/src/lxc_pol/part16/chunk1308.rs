//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 1308/1361 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk1308<F: Float>(t52930: F, t52961: F, t52968: F, t14311: F, t3083: F, t50927: F, t50944: F, t50949: F, t52940: F, t52944: F, t52952: F, t52956: F, t52959: F, t52976: F, t52982: F, t52986: F) -> F {
    let t54896 = F::cast_from(7.0_f64) / F::cast_from(72.0_f64) * t52930;
    let t54902 = F::cast_from(7.0_f64) / F::cast_from(1152.0_f64) * t52961;
    let t54904 = F::cast_from(7.0_f64) / F::cast_from(576.0_f64) * t52968;
    let t54911 = F::cast_from(7.0_f64) / F::cast_from(144.0_f64) * t3083 * t14311;
    let t54912 = -t54896 + t52940 / F::cast_from(192.0_f64) + t52944 / F::cast_from(384.0_f64) - t52952 / F::cast_from(1536.0_f64) + t52956 / F::cast_from(384.0_f64) - t52959 / F::cast_from(96.0_f64) - t54902 + F::cast_from(7.0_f64) / F::cast_from(576.0_f64) * t50927 + t54904 + t52976 / F::cast_from(384.0_f64) - t52982 / F::cast_from(96.0_f64) - t52986 / F::cast_from(96.0_f64) + F::cast_from(7.0_f64) / F::cast_from(72.0_f64) * t50944 + F::cast_from(119.0_f64) / F::cast_from(1728.0_f64) * t50949 + t54911;
    t54912
}
