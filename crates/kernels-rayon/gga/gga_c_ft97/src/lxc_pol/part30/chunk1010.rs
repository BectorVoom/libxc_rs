//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 1010/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk1010(t150228: f64, t193: f64, t2506: f64, t6109: f64, t1424: f64, t27742: f64, t1434: f64, t3821: f64, t7440: f64, t9942: f64, t35318: f64, t42500: f64, t446: f64, t713: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t150231 = t6109 * t193 * t2506 * t150228;
    let t150233 = t1424 * t27742;
    let t150236 = t1434 * t193 * t2506 * t150233;
    let t150238 = t7440 * t3821;
    let t150241 = t1434 * t193 * t9942 * t150238;
    let t150246 = t446 * t42500 * t35318 * t713;
    (t150231, t150233, t150236, t150238, t150241, t150246)
}
