//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 17 (v4rho3sigma_5) CSE chunk 618/1352 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part17_v4rho3sigma_5_chunk618<F: Float>(t1459: F, t1463: F, t1467: F, t1471: F, t1475: F, t1482: F, t1483: F, t1486: F, t2042: F, t279: F, t2852: F, t2857: F, t2858: F, t2922: F, t2926: F, t481: F, t526: F) -> F {
    let t2928 = t2852 * t279 - F::cast_from(0.54045904796391420712e-1_f64) * t2042 - F::cast_from(0.29056741517886919367e-3_f64) * t1459 - t1463 + t1467 + t1471 - t1475 - t1482 + F::cast_from(0.19957056683757681823e-1_f64) * t1483 + t1486 + F::new(6.0) * t2857 * t2858 * t481 + t2922 * t526 - F::cast_from(0.54045904796391420712e-1_f64) * t2926;
    t2928
}
