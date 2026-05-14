//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 627/1100 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk627<F: Float>(t5117: F, t1783: F, t663: F, t4937: F, t4984: F, t4986: F, t4987: F, t4990: F, t4994: F, t4997: F, t5000: F, t5007: F, t5012: F, t5017: F, t5021: F, t5101: F, t5104: F, t5107: F, t5115: F) -> (F, F, F) {
    let t5118 = 8.0 / 45.0 * t5117;
    let t5120 = 4.0 / 5.0 * t1783 * t663;
    let t5121 = t4937 + t4984 + t4986 - 2.0 / 15.0 * t4987 - t4990 - t4994 - t4997 - t5000 - t5007 - t5012 - t5017 - t5021 - t5101 + t5104 + t5107 - t5115 - t5118 - t5120;
    (t5118, t5120, t5121)
}
