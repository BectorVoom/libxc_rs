//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 669/1100 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk669<F: Float>(t169: F, t301: F, t366: F, t745: F, t1354: F, t532: F, t159: F, t285: F, t1500: F, t2036: F, t1504: F, t481: F, t100: F, t143: F, t2035: F, t281: F, t475: F, t523: F, t5636: F, t5645: F, t5650: F, t5653: F, t5657: F, t5661: F, t5666: F, t5670: F) -> (F, F, F, F, F) {
    let t5674 = t169 * t366 * t745 * t301;
    let t5676 = t532 * t1354;
    let t5678 = t5676 * t159 * t285;
    let t5680 = t1500 * t2036;
    let t5683 = t1504 * t481;
    let t5684 = t5683 * t100;
    let t5687 = -0.11974234010254609094e-1 * t281 * t5636 + 3.0 * t475 * t143 * t5645 - 9.0 * t5650 * t5653 + 9.0 * t2035 * t5657 - 2.0 * t523 * t5661 - 0.16213771438917426213e0 * t5666 + 0.40679438125041687114e-2 * t5670 + 0.59450495276030562782e0 * t5674 - 0.87170224553660758101e-3 * t5678 + 9.0 * t2035 * t5680 + 6.0 * t5684 * t143;
    (t5676, t5680, t5683, t5684, t5687)
}
