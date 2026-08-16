//! GGA_C_GAPLOC lxc pol — lxc_pol part 49 (v4rhosigma3_14) CSE chunk 1149/1217 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part49_v4rhosigma3_14_chunk1149(t41477: f64, t1445: f64, t1998: f64, t47270: f64, t701: f64, t326: f64, t47243: f64, t825: f64, t2684: f64, t7585: f64, t44152: f64, t44154: f64, t44155: f64, t44156: f64, t44159: f64, t44162: f64, t44164: f64, t44167: f64) -> f64 {
    let t47558 = 0.12780975317973583226e0_f64 * t41477;
    let t47562 = 0.23005755572352449806e1_f64 * t1998 * t1445 * t47270 * t701;
    let t47564 = t825 * t326 * t47243;
    let t47567 = t2684 * t7585 * t47243;
    let t47571 = -t44152 - t44154 + t44155 - t44156 - t47558 - t47562 - 0.92023022289409799224e1_f64 * t47564 + 0.43710935587469654631e2_f64 * t47567 + 0.47667319935800568892e0_f64 * t44159 - t44162 - t44164 - 0.69017266717057349418e1_f64 * t44167;
    t47571
}
