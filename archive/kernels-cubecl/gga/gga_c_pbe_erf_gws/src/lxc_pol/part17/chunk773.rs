//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 17 (v4rho3sigma_5) CSE chunk 773/1352 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part17_v4rho3sigma_5_chunk773<F: Float>(t153: F, t274: F, t4573: F, t1592: F, t475: F, t142: F, t1504: F, t525: F, t1354: F, t285: F, t545: F, t281: F) -> (F, F, F, F) {
    let t5595 = F::cast_from(0.4429070076315393047e1_f64) * t153 * t4573 * t274;
    let t5598 = t475 * t1592;
    let t5602 = t142 * t1504;
    let t5603 = t525 * t5602;
    let t5607 = t1354 * t545 * t285;
    let t5608 = t281 * t5607;
    (t5595, t5598, t5603, t5608)
}
