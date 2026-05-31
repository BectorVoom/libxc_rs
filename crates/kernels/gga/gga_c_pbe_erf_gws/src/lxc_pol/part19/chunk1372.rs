//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 1372/1404 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk1372<F: Float>(t14911: F, t3083: F, t15500: F, t4414: F, t14918: F, t3040: F, t4083: F, t55375: F, t55722: F, t56815: F, t56818: F, t56821: F, t56836: F, t56840: F, t56843: F, t56847: F, t56849: F, t56853: F, t8793: F, t9958: F) -> F {
    let t58449 = t3083 * t14911;
    let t58457 = t4414 * t15500;
    let t58465 = t56815 / F::cast_from(4.0_f64) - t9958 * t4083 / F::cast_from(96.0_f64) + F::cast_from(7.0_f64) / F::cast_from(144.0_f64) * t58449 - t3040 * t14918 / F::cast_from(48.0_f64) + t8793 * t55722 / F::cast_from(24.0_f64) + t56818 / F::cast_from(96.0_f64) + t56821 / F::cast_from(96.0_f64) - F::cast_from(7.0_f64) / F::cast_from(72.0_f64) * t58457 - F::cast_from(5.0_f64) / F::cast_from(64.0_f64) * t56836 - t56840 / F::cast_from(256.0_f64) - t56843 / F::cast_from(24.0_f64) - t56847 / F::cast_from(384.0_f64) - t56849 / F::cast_from(48.0_f64) + t56853 / F::cast_from(192.0_f64) - t55375;
    t58465
}
