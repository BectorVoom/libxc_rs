//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 697/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk697<F: Float>(t4487: F, t1650: F, t219: F, t1265: F, t1656: F, t3365: F, t220: F, t3255: F, t73: F, t1639: F, t532: F, t1219: F, param_beta: F) -> (F, F, F, F, F, F) {
    let t4488 = param_beta * t4487;
    let t4490 = t1650 * t219;
    let t4493 = t1656 * t1265;
    let t4494 = t3365 * t4493;
    let t4498 = t220 * t73 * t3255;
    let t4499 = t532 * t1639;
    let t4508 = t220 * t73 * t1219;
    (t4488, t4490, t4494, t4498, t4499, t4508)
}
