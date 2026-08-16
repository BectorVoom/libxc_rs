//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 50 (v4rho2sigma2_6) CSE chunk 1273/1294 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part50_v4rho2sigma2_6_chunk1273<F: Float>(t120544: F, t6888: F, t6891: F, t114299: F, t114285: F, t26331: F, t26333: F, t114316: F, t32769: F, t6883: F, t1985: F, t26193: F, t31123: F) -> (F, F, F, F, F, F) {
    let t120616 = F::cast_from(0.3289868133696452873e-1_f64) * t6888 * t120544 * t6891;
    let t120621 = F::cast_from(0.82246703342411321825e-2_f64) * t114299;
    let t120628 = F::cast_from(0.9869604401089358619e-1_f64) * t26331 * t114285 * t26333;
    let t120629 = F::cast_from(0.16449340668482264365e-1_f64) * t114316;
    let t120632 = t6883 * t32769;
    let t120633 = F::cast_from(0.38381794893125283518e-1_f64) * t120632;
    let t120641 = F::cast_from(0.16449340668482264365e-1_f64) * t1985 * t26193 * t31123;
    (t120616, t120621, t120628, t120629, t120633, t120641)
}
