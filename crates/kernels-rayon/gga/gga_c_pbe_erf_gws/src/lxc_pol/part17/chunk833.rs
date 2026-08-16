//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 17 (v4rho3sigma_5) CSE chunk 833/1352 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part17_v4rho3sigma_5_chunk833(t1069: f64, t1617: f64, t1627: f64, t2603: f64, t2722: f64, t649: f64, t617: f64, t1621: f64, t1620: f64, t4906: f64, t4911: f64, t4936: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t6998 = t1069 * t1617;
    let t7002 = 8.0_f64 / 15.0_f64 * t1627 * t2603;
    let t7003 = t649 * t2722;
    let t7004 = t7003 * t617;
    let t7005 = t1621 * t7004;
    let t7007 = 8.0_f64 / 15.0_f64 * t1620 * t7005;
    let t7008 = 4.0_f64 / 45.0_f64 * t4906;
    let t7009 = 4.0_f64 / 45.0_f64 * t4911;
    let t7010 = 32.0_f64 / 135.0_f64 * t4936;
    (t6998, t7002, t7007, t7008, t7009, t7010)
}
