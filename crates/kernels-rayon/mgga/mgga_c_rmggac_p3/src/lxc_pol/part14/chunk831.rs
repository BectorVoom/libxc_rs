//! MGGA_C_RMGGAC lxc pol — lxc_pol part 14 (v4rho3sigma_5) CSE chunk 831/1089 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part14_v4rho3sigma_5_chunk831(t1587: f64, t236: f64, t3352: f64, t495: f64, t7230: f64, t1528: f64, t3351: f64, t498: f64, t9210: f64, t38539: f64, t38541: f64, t38545: f64, t38550: f64, t38552: f64, t38554: f64, t38556: f64, t38560: f64, t38563: f64, t38566: f64, t38570: f64, t38572: f64, t38574: f64, t38576: f64, t38578: f64) -> f64 {
    let t38583 = t7230 * t3352 * t236 * t1587 * t495;
    let t38588 = t3351 * t9210 * t236 * t1528 * t498;
    let t38590 = -0.31923449919973379548e-4_f64 * t38539 + 0.85129199786595678796e-5_f64 * t38541 + 0.85129199786595678796e-5_f64 * t38545 + 0.31923449919973379548e-4_f64 * t38550 + 0.30487649791575028314e-3_f64 * t38552 + 0.30487649791575028314e-3_f64 * t38554 - 0.35220688045884876043e-2_f64 * t38556 - t38560 - t38563 + 0.20455996240684006296e0_f64 * t38566 - 0.72732431077987577942e-1_f64 * t38570 + 0.51077519871957407276e-4_f64 * t38572 - 0.76616279807936110914e-4_f64 * t38574 - 0.25538759935978703638e-4_f64 * t38576 + 0.25538759935978703638e-4_f64 * t38578 - 0.31923449919973379548e-4_f64 * t38583 + 0.17025839957319135759e-4_f64 * t38588;
    t38590
}
