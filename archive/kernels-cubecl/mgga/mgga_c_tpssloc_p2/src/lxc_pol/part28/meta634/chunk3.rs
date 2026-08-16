//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 2010/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk2010<F: Float>(t90604: F, t90609: F, t16030: F, t24082: F, t24088: F, t24095: F, t24147: F, t26996: F, t3758: F, t5215: F, t5321: F, t5326: F, t7199: F, t80738: F, t84400: F, t90626: F, t90634: F) -> (F, F, F) {
    let t93404 = F::cast_from(0.76763589786250567036e-1_f64) * t90604;
    let t93407 = F::cast_from(0.9869604401089358619e-1_f64) * t90609;
    let t93431 = -F::cast_from(0.82246703342411321825e-2_f64) * t80738 - t84400 + F::cast_from(0.16449340668482264365e-1_f64) * t90626 + F::cast_from(4.0_f64) * t5215 * t24147 + F::cast_from(2.0_f64) * t5215 * t24088 + F::cast_from(4.0_f64) * t24082 * t5326 + F::cast_from(4.0_f64) * t24095 * t5326 + F::cast_from(4.0_f64) * t16030 * t7199 + F::cast_from(2.0_f64) * t5321 * t24088 - F::cast_from(0.9869604401089358619e-1_f64) * t90634 + F::cast_from(4.0_f64) * t3758 * t26996;
    (t93404, t93407, t93431)
}
