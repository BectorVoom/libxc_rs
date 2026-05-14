//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1045/1100 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk1045<F: Float>(t18467: F, t18471: F, t18474: F, t18477: F, t18479: F, t18512: F, t18514: F, t18518: F, t18521: F, t18523: F, t18527: F, t18529: F, t19482: F, t19488: F, t18531: F, t18535: F, t18537: F, t18539: F, t18541: F, t18556: F, t18562: F, t18567: F, t18571: F, t18574: F, t18577: F, t18580: F, t18583: F, t18587: F) -> (F, F) {
    let t20975 = t18467 - t18471 - t18474 + t18477 + t18479 + t18512 - t18514 + t19482 + t18518 + t18521 - t18523 - t19488 + t18527 + t18529;
    let t20977 = t18531 - t18535 + t18537 + t18539 - t18541 - t18556 - t18562 + t18567 + t18571 - t18574 + t18577 + t18580 + t18583 + t18587;
    (t20975, t20977)
}
