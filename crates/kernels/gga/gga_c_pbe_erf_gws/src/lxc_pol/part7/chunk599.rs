//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 599/1100 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk599<F: Float>(t43: F, t50: F, t1336: F, t461: F, t428: F, t726: F, t1402: F, t418: F, t1407: F, t4352: F, t4360: F, t47: F, t728: F, t1412: F, t422: F, t1416: F, t4367: F, t4373: F, t52: F, zeta_threshold: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t44 = t43 <= zeta_threshold;
    let t51 = t50 <= zeta_threshold;
    let t4753 = t1336 * t461;
    let t4754 = 36.0 * t4753;
    let t4755 = t1336 * t428;
    let t4756 = 36.0 * t4755;
    let t4757 = 1.0 / t726;
    let t4760 = t1402 * t418;
    let t4766 = piecewise3(t44, 0.0, -8.0 / 27.0 * t4757 * t4352 + 4.0 / 3.0 * t4760 * t1407 + 4.0 / 3.0 * t47 * t4360);
    let t4767 = 1.0 / t728;
    let t4770 = t1412 * t422;
    let t4776 = piecewise3(t51, 0.0, -8.0 / 27.0 * t4767 * t4367 + 4.0 / 3.0 * t4770 * t1416 + 4.0 / 3.0 * t52 * t4373);
    (t4753, t4754, t4755, t4756, t4757, t4760, t4766, t4767, t4770, t4776)
}
