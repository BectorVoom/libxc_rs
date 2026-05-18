//! MGGA_C_REVTPSS lxc pol — lxc_pol part 52 (v4rho2sigma2_7) CSE chunk 1098/1292 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part52_v4rho2sigma2_7_chunk1098<F: Float>(t2056: F, t34258: F, t7741: F, t94: F, t7978: F, t8634: F, t5542: F, t8714: F, t2014: F, t7898: F, t8718: F, t196: F, t197: F, t8075: F) -> (F, F, F, F, F, F, F, F) {
    let t34260 = F::new(2.0) * t34258 * t2056;
    let t34261 = t94 * t7741;
    let t34263 = F::new(2.0) * t34261 * t2056;
    let t34265 = F::new(2.0) * t8634 * t7978;
    let t34266 = t8714 * t5542;
    let t34267 = t2014 * t34266;
    let t34268 = t7898 * t8718;
    let t34270 = t8075 * t196 * t197;
    (t34260, t34261, t34263, t34265, t34266, t34267, t34268, t34270)
}
