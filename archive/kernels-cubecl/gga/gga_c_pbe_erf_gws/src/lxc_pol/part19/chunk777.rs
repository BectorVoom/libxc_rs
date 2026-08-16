//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 777/1404 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk777<F: Float>(t1365: F, t153: F, t745: F, t1464: F, t242: F, t366: F, t5: F, t168: F, t270: F, t274: F, t4573: F, t1503: F, t522: F) -> (F, F, F, F, F, F) {
    let t5585 = t153 * t1365 * t745;
    let t5588 = F::cast_from(0.50257692321302641125e0_f64) * t1464 * t242;
    let t5589 = t5 * t366;
    let t5592 = F::cast_from(0.19455129084526283664e0_f64) * t168 * t5589 * t270;
    let t5595 = F::cast_from(0.4429070076315393047e1_f64) * t153 * t4573 * t274;
    let t5601 = t1503 * t522;
    (t5585, t5588, t5589, t5592, t5595, t5601)
}
