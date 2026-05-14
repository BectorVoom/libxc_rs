//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 731/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk731<F: Float>(t1452: F, t153: F, t542: F, t1457: F, t242: F, t1365: F, t745: F, t1464: F, t366: F, t5: F, t168: F, t270: F, t274: F, t4573: F, t1592: F, t475: F) -> (F, F, F, F, F, F, F, F) {
    let t5580 = t153 * t542 * t1452;
    let t5582 = t1457 * t242;
    let t5585 = t153 * t1365 * t745;
    let t5588 = 0.50257692321302641125e0 * t1464 * t242;
    let t5589 = t5 * t366;
    let t5592 = 0.19455129084526283664e0 * t168 * t5589 * t270;
    let t5595 = 0.4429070076315393047e1 * t153 * t4573 * t274;
    let t5598 = t475 * t1592;
    (t5580, t5582, t5585, t5588, t5589, t5592, t5595, t5598)
}
