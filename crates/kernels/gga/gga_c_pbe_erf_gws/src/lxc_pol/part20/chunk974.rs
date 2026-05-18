//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 974/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk974<F: Float>(t10995: F, t5218: F, t7148: F, t995: F, t2555: F, t7811: F, t10959: F, t10963: F, t10967: F, t10971: F, t10974: F, t10975: F, t10977: F, t10981: F, t10984: F, t10987: F, t10991: F, t10994: F, t5521: F, t7810: F) -> (F, F, F, F) {
    let t10997 = F::new(16.0) / F::new(45.0) * t5218 * t10995;
    let t10998 = t7148 * t995;
    let t10999 = t10998 * t2555;
    let t11001 = F::new(32.0) / F::new(45.0) * t5218 * t10999;
    let t11002 = F::new(8.0) / F::new(135.0) * t7811;
    let t11003 = -t10959 - t10963 - t10967 + t10971 + t10974 - t10975 - t7810 - t5521 - t10977 + t10981 + t10984 - t10987 + t10991 - t10994 - t10997 - t11001 - t11002;
    (t10997, t11001, t11002, t11003)
}
