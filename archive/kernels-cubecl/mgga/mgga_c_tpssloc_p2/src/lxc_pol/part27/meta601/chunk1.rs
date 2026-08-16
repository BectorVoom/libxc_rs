//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 2070/2372 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2070<F: Float>(t23337: F, t82431: F, t10336: F, t1920: F, t1922: F, t23391: F, t6680: F, t3173: F, t3175: F, t1921: F, t1054: F, t3206: F) -> (F, F, F, F, F) {
    let t82432 = t82431 * t23337;
    let t82436 = F::cast_from(0.30461741978670859935e-2_f64) * t1920 * t10336 * t1922;
    let t82437 = t6680 * t23391;
    let t82441 = t3173 * t3175;
    let t82442 = t1921 * t82441;
    let t82457 = t1054 * t3206;
    (t82432, t82436, t82437, t82442, t82457)
}
