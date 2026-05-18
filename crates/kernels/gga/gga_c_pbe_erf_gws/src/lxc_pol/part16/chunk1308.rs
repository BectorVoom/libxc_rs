//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 1308/1361 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk1308<F: Float>(t52930: F, t52961: F, t52968: F, t14311: F, t3083: F, t50927: F, t50944: F, t50949: F, t52940: F, t52944: F, t52952: F, t52956: F, t52959: F, t52976: F, t52982: F, t52986: F) -> F {
    let t54896 = F::new(7.0) / F::new(72.0) * t52930;
    let t54902 = F::new(7.0) / F::new(1152.0) * t52961;
    let t54904 = F::new(7.0) / F::new(576.0) * t52968;
    let t54911 = F::new(7.0) / F::new(144.0) * t3083 * t14311;
    let t54912 = -t54896 + t52940 / F::new(192.0) + t52944 / F::new(384.0) - t52952 / F::new(1536.0) + t52956 / F::new(384.0) - t52959 / F::new(96.0) - t54902 + F::new(7.0) / F::new(576.0) * t50927 + t54904 + t52976 / F::new(384.0) - t52982 / F::new(96.0) - t52986 / F::new(96.0) + F::new(7.0) / F::new(72.0) * t50944 + F::new(119.0) / F::new(1728.0) * t50949 + t54911;
    t54912
}
