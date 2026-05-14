//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 750/1141 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk750<F: Float>(t108: F, t12339: F, t12345: F, t12350: F, t12355: F, t2538: F, t2544: F, t3346: F, t3354: F, t476: F, t478: F, t726: F, t728: F, t12803: F, t12807: F, t12812: F, t12816: F, t12820: F, t12824: F, t12825: F, t12827: F, t12828: F, t12832: F, t12834: F, t12873: F, t12875: F, t12876: F, t12877: F, t12878: F, t267: F, t5562: F) -> (F, F) {
    let t13039 = (40.0 / 27.0 * t476 * t12339 + 20.0 / 3.0 * t2538 * t3346 + 4.0 / 3.0 * t726 * t12345 + 40.0 / 27.0 * t478 * t12350 + 20.0 / 3.0 * t2544 * t3354 + 4.0 / 3.0 * t728 * t12355) * t108;
    let t13042 = t12803 - t12807 - t12812 + t12816 - t12820 + t12824 + t12825 + t12827 - t12828 - t12832 + t12834 - t12873 - t12875 - t12876 + t12877 + t5562 - t13039 * t267 / 15.0 - t12878;
    (t13039, t13042)
}
