//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 931/1404 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk931<F: Float>(t10442: F, t418: F, t2559: F, t587: F, t3421: F, t562: F, t1820: F, t1022: F, t7490: F, t2679: F, t5211: F, t10401: F, t617: F, t7499: F) -> (F, F, F, F, F) {
    let t10443 = t10442 * t418;
    let t10444 = t2559 * t10443;
    let t10446 = F::cast_from(4.0_f64) / F::cast_from(27.0_f64) * t587 * t10444;
    let t10447 = t3421 * t562;
    let t10448 = t2559 * t10447;
    let t10450 = F::cast_from(8.0_f64) / F::cast_from(27.0_f64) * t1820 * t10448;
    let t10451 = t7490 * t1022;
    let t10452 = t10451 * t2679;
    let t10454 = F::cast_from(16.0_f64) / F::cast_from(27.0_f64) * t5211 * t10452;
    let t10456 = t7499 * t10401 * t617;
    (t10443, t10446, t10450, t10454, t10456)
}
