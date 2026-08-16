//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 789/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk789(t10914: f64, t10925: f64, t10928: f64, t10931: f64, t10933: f64, t10973: f64, t10993: f64, t7811: f64, t12476: f64, t1821: f64, t587: f64, t12468: f64, t2559: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t12786 = 16.0_f64 / 15.0_f64 * t10914;
    let t12787 = 8.0_f64 / 27.0_f64 * t10925;
    let t12788 = 8.0_f64 / 45.0_f64 * t10928;
    let t12789 = 16.0_f64 / 15.0_f64 * t10931;
    let t12790 = 16.0_f64 / 45.0_f64 * t10933;
    let t12791 = 8.0_f64 / 15.0_f64 * t10973;
    let t12792 = 16.0_f64 / 45.0_f64 * t10993;
    let t12793 = 4.0_f64 / 45.0_f64 * t7811;
    let t12794 = t1821 * t12476;
    let t12796 = 8.0_f64 / 15.0_f64 * t587 * t12794;
    let t12797 = t2559 * t12468;
    (t12786, t12787, t12788, t12789, t12790, t12791, t12792, t12793, t12794, t12796, t12797)
}
