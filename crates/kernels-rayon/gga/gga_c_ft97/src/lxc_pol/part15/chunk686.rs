//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 686/1222 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk686(t20098: f64, t370: f64, t27: f64, t89: f64, t4436: f64, t925: f64, t7824: f64, t446: f64, t4458: f64, t942: f64, t1564: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t20099 = t370 * t20098;
    let t20101 = t89 * t27 * t20099;
    let t20103 = t925 * t4436;
    let t20104 = t7824 * t20103;
    let t20105 = t446 * t20104;
    let t20107 = t4458 * t942;
    let t20108 = t1564 * t20107;
    let t20109 = t446 * t20108;
    let t20113 = t4436 * t942;
    (t20099, t20101, t20103, t20104, t20105, t20107, t20108, t20109, t20113)
}
