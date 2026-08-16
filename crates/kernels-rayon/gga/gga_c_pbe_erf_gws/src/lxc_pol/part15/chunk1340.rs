//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 15 (v4rho3sigma_3) CSE chunk 1340/1352 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part15_v4rho3sigma_3_chunk1340(t1113: f64, t28947: f64, t3972: f64, t3975: f64, t1161: f64, t874: f64, t13776: f64, t2171: f64, t50956: f64, t3102: f64, t859: f64, t13792: f64) -> (f64, f64, f64) {
    let t54588 = t3972 * t3975 * t1113 * t28947;
    let t54590 = t1161 * t874;
    let t54593 = t13776 * t50956 * t54590 * t2171;
    let t54595 = t859 * t3102;
    let t54596 = t13792 * t54595;
    (t54588, t54593, t54596)
}
