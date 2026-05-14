//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 863/1100 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk863<F: Float>(t17577: F, t186: F, t220: F, t616: F, t1726: F, t1750: F, t4908: F, t633: F, t17559: F, t17562: F, t17565: F, t17567: F, t17569: F, t17571: F, t17573: F, t17575: F) -> (F, F, F, F) {
    let t17581 = 4.0 / 15.0 * t616 * t186 * t220 * t17577;
    let t17583 = 4.0 / 5.0 * t1750 * t1726;
    let t17584 = t633 * t4908;
    let t17585 = 64.0 / 405.0 * t17584;
    let t17586 = t17559 + t17562 - t17565 + t17567 + t17569 + t17571 + t17573 + t17575 + t17581 - t17583 + t17585;
    (t17581, t17583, t17585, t17586)
}
