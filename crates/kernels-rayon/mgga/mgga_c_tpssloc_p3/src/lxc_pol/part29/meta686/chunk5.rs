//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 2355/2357 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2355(t1396: f64, t1398: f64, t1404: f64, t1852: f64, t1858: f64, t24955: f64, t24977: f64, t27908: f64, t27930: f64, t85403: f64, t85407: f64, t85412: f64, t86557: f64, t86559: f64, t96300: f64, t96303: f64, t96308: f64, t96327: f64, t96337: f64) -> f64 {
    let t96340 = t1852 * t24977 + t96300 + t86557 + t85412 + 2.0_f64 * t86559 + t96303 + t24955 * t1858 + t85407 + 2.0_f64 * t27908 * t1404 + t96308 + t85403 + 2.0_f64 * t1396 * t27930 + t1398 * (t96327 + t96337);
    t96340
}
