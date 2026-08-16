//! GGA_C_GAPC lxc pol — lxc_pol part 38 (v4rho2sigma2_17) CSE chunk 695/1307 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part38_v4rho2sigma2_17_chunk695(t1: f64, t350: f64, t786: f64, t961: f64, t2530: f64, t311: f64, t6851: f64, t442: f64, t6856: f64, t277: f64, t4978: f64, t2188: f64, t329: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t7943 = t786 * t1 * t350;
    let t7944 = t961 * t7943;
    let t7949 = t2530 * t350;
    let t7953 = t311 * t6851;
    let t7956 = t6856 * t442;
    let t7967 = t277 * t4978;
    let t7974 = t2188 * t329;
    (t7943, t7944, t7949, t7953, t7956, t7967, t7974)
}
