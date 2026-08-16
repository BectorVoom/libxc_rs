//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 15 (v4rho3sigma_3) CSE chunk 618/1352 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part15_v4rho3sigma_3_chunk618(t1459: f64, t1463: f64, t1467: f64, t1471: f64, t1475: f64, t1482: f64, t1483: f64, t1486: f64, t2042: f64, t279: f64, t2852: f64, t2857: f64, t2858: f64, t2922: f64, t2926: f64, t481: f64, t526: f64) -> f64 {
    let t2928 = t2852 * t279 - 0.54045904796391420712e-1_f64 * t2042 - 0.29056741517886919367e-3_f64 * t1459 - t1463 + t1467 + t1471 - t1475 - t1482 + 0.19957056683757681823e-1_f64 * t1483 + t1486 + 6.0_f64 * t2857 * t2858 * t481 + t2922 * t526 - 0.54045904796391420712e-1_f64 * t2926;
    t2928
}
