//! GGA_C_GAPLOC lxc pol — lxc_pol part 46 (v4rhosigma3_11) CSE chunk 822/1029 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part46_v4rhosigma3_11_chunk822(t10473: f64, t2478: f64, t6576: f64, t4130: f64, t41596: f64, t4781: f64, t590: f64, t34688: f64, t9272: f64, t9273: f64, t18313: f64, t31119: f64, t3394: f64) -> (f64, f64, f64, f64) {
    let t41649 = t6576 * t10473 * t2478;
    let t41650 = 0.76685851907841499353e0_f64 * t41649;
    let t41654 = 0.13803453343411469884e2_f64 * t4781 * t4130 * t41596 * t590;
    let t41656 = t9272 * t34688 * t9273;
    let t41657 = 0.10352590007558602413e2_f64 * t41656;
    let t41660 = t31119 * t18313 * t3394 * t9273;
    (t41650, t41654, t41657, t41660)
}
