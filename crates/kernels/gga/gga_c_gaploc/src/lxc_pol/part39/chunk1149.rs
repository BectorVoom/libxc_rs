//! GGA_C_GAPLOC lxc pol — lxc_pol part 39 (v4rhosigma3_4) CSE chunk 1149/1217 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part39_v4rhosigma3_4_chunk1149<F: Float>(t41477: F, t1445: F, t1998: F, t47270: F, t701: F, t326: F, t47243: F, t825: F, t2684: F, t7585: F, t44152: F, t44154: F, t44155: F, t44156: F, t44159: F, t44162: F, t44164: F, t44167: F) -> F {
    let t47558 = F::cast_from(0.12780975317973583226e0_f64) * t41477;
    let t47562 = F::cast_from(0.23005755572352449806e1_f64) * t1998 * t1445 * t47270 * t701;
    let t47564 = t825 * t326 * t47243;
    let t47567 = t2684 * t7585 * t47243;
    let t47571 = -t44152 - t44154 + t44155 - t44156 - t47558 - t47562 - F::cast_from(0.92023022289409799224e1_f64) * t47564 + F::cast_from(0.43710935587469654631e2_f64) * t47567 + F::cast_from(0.47667319935800568892e0_f64) * t44159 - t44162 - t44164 - F::cast_from(0.69017266717057349418e1_f64) * t44167;
    t47571
}
