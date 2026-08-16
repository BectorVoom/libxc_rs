//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 1381/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1381<F: Float>(t11135: F, t11203: F, t1124: F, t3356: F, t3355: F, t432: F, t427: F) -> (F, F, F, F, F) {
    let t11369 = F::cast_from(0.93932222222222222223e0_f64) * t11135;
    let t11372 = F::cast_from(0.36793333333333333333e0_f64) * t11203;
    let t11415 = t1124 * t3356;
    let t11419 = F::cast_from(1.0_f64) / t3355 / t432;
    let t11420 = t427 * t11419;
    (t11369, t11372, t11415, t11419, t11420)
}
