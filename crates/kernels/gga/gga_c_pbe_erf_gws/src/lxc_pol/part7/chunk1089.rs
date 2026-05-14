//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1089/1100 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk1089<F: Float>(t21387: F, t21395: F, t21412: F, t21414: F, t21424: F, t21429: F, t21445: F, t21455: F, t21462: F, t21478: F, t21494: F, t21502: F, t21513: F, t21528: F, t21534: F, t21540: F, t21544: F, t21563: F, t21565: F, t21577: F, t21580: F, t21594: F) -> (F, F) {
    let t21709 = -t21387 - t21395 + t21412 - t21414 - t21424 + t21429 + t21445 - t21455 + t21462 - t21478 - t21494;
    let t21711 = -t21502 + t21513 + t21528 - t21534 - t21540 + t21544 - t21563 - t21565 + t21577 + t21580 - t21594;
    (t21709, t21711)
}
