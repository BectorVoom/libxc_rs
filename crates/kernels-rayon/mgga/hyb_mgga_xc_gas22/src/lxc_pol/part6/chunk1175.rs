//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1175/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1175(t345: f64, t2597: f64, t2600: f64, t2476: f64, t2519: f64, t347: f64, t21389: f64, t2470: f64, t2520: f64, t21424: f64, t2575: f64, t2569: f64, t2576: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t21474 = f64::powf(t345, -0.25e1_f64);
    let t21502 = t2597 * t2597;
    let t21503 = 1.0_f64 / t21502;
    let t21506 = t2600 * t2600;
    let t21507 = 1.0_f64 / t21506;
    let t21537 = t347 / t2519 / t2476;
    let t21541 = 0.96141975308641975307e-1_f64 * t21389;
    let t21552 = t2470 * t2520;
    let t21557 = 0.31003950617283950618e1_f64 * t21389;
    let t21560 = 0.13388493827160493828e1_f64 * t21424;
    let t21587 = 0.18467901234567901234e0_f64 * t21389;
    let t21601 = 1.0_f64 / t2597 / t2575;
    let t21613 = t2569 * t2576;
    (t21474, t21503, t21507, t21537, t21541, t21552, t21557, t21560, t21587, t21601, t21613)
}
