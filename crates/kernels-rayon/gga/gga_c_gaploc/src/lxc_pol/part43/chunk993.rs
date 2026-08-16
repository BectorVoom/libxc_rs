//! GGA_C_GAPLOC lxc pol — lxc_pol part 43 (v4rhosigma3_8) CSE chunk 993/1072 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part43_v4rhosigma3_8_chunk993(t204: f64, t47803: f64, t587: f64, t2487: f64, t6711: f64, t4130: f64, t46849: f64, t4781: f64, t590: f64, t13750: f64, t1441: f64, t1339: f64, t13749: f64, t1537: f64) -> (f64, f64, f64, f64, f64) {
    let t47805 = t587 * t204 * t47803;
    let t47808 = t2487 * t6711 * t47803;
    let t47812 = t4781 * t4130 * t46849 * t590;
    let t47823 = 0.51123901271894332902e0_f64 * t1441 * t13750 * t590;
    let t47827 = 0.51123901271894332902e0_f64 * t1537 * t1339 * t13749 * t590;
    (t47805, t47808, t47812, t47823, t47827)
}
