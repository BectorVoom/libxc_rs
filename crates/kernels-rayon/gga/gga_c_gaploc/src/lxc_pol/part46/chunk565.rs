//! GGA_C_GAPLOC lxc pol — lxc_pol part 46 (v4rhosigma3_11) CSE chunk 565/1029 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part46_v4rhosigma3_11_chunk565(t1445: f64, t9954: f64, t1457: f64, t9604: f64, t9591: f64, t1: f64, t3234: f64, t106: f64, t316: f64, t3276: f64, t773: f64, t1645: f64, t2586: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t9955 = t1445 * t9954;
    let t9958 = t1457 * t9604;
    let t9961 = t1457 * t9591;
    let t9964 = t3234 * t1;
    let t9965 = t9964 * t106;
    let t9966 = t9965 * t316;
    let t9969 = t773 * t3276;
    let t9972 = t1645 * t2586;
    (t9955, t9958, t9961, t9966, t9969, t9972)
}
