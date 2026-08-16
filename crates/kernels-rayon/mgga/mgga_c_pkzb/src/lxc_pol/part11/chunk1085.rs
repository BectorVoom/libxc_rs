//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1085/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1085(t2316: f64, t2319: f64, t2294: f64, t18439: f64, t18442: f64, t2238: f64, t338: f64, t2241: f64, t6198: f64, t828: f64, t2195: f64, t2411: f64, t54: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t18508 = t2316 * t2316;
    let t18509 = 1.0_f64 / t18508;
    let t18512 = t2319 * t2319;
    let t18513 = 1.0_f64 / t18512;
    let t18520 = 1.0_f64 / t2316 / t2294;
    let t18554 = 0.31003950617283950618e1_f64 * t18439;
    let t18555 = 0.13388493827160493828e1_f64 * t18442;
    let t18587 = t2238 * t2238;
    let t18589 = t338 / t18587;
    let t18591 = t2241 * t2241;
    let t18592 = 1.0_f64 / t18591;
    let t18596 = 0.96141975308641975307e-1_f64 * t18439;
    let t18612 = t828 * t6198;
    let t18617 = t338 / t2238 / t2195;
    let t18657 = t54 * t2411;
    (t18509, t18513, t18520, t18554, t18555, t18589, t18592, t18596, t18612, t18617, t18657)
}
