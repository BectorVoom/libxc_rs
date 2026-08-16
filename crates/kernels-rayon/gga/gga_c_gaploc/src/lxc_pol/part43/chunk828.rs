//! GGA_C_GAPLOC lxc pol — lxc_pol part 43 (v4rhosigma3_8) CSE chunk 828/1072 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part43_v4rhosigma3_8_chunk828(t41626: f64, t2365: f64, t31591: f64, t4391: f64, t12960: f64, t31051: f64, t10473: f64, t2478: f64, t6576: f64, t4130: f64, t41596: f64, t4781: f64, t590: f64) -> (f64, f64, f64, f64, f64) {
    let t41627 = 0.59584149919750711116e-1_f64 * t41626;
    let t41629 = t4391 * t2365 * t31591;
    let t41630 = 0.59584149919750711116e-1_f64 * t41629;
    let t41645 = t31051 * t12960;
    let t41646 = 0.19171462976960374838e1_f64 * t41645;
    let t41649 = t6576 * t10473 * t2478;
    let t41654 = 0.13803453343411469884e2_f64 * t4781 * t4130 * t41596 * t590;
    (t41627, t41630, t41646, t41649, t41654)
}
