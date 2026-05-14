//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 1218/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk1218<F: Float>(t14918: F, t3083: F, t1115: F, t12201: F, t14311: F, t14327: F, t3917: F, t4083: F, t54882: F, t55962: F, t57687: F, t57689: F, t57694: F, t57696: F, t57700: F, t57702: F, t57705: F, t57707: F, t57711: F) -> (F,) {
    let t58929 = t3083 * t14918;
    let t58940 = t57687 / 12.0 - 7.0 / 24.0 * t57689 - t57694 / 12.0 + 7.0 / 36.0 * t57696 - t12201 * t4083 / 96.0 - t1115 * t54882 / 48.0 + 7.0 / 144.0 * t58929 - t3917 * t14311 / 96.0 - t3917 * t14327 / 96.0 - t57700 / 384.0 + 7.0 / 72.0 * t57702 - t57705 / 12.0 - t55962 - 7.0 / 144.0 * t57707 + t57711 / 384.0;
    (t58940,)
}
