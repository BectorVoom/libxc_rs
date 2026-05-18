//! GGA_C_GAPLOC lxc pol — lxc_pol part 27 (v4rho2sigma2_10) CSE chunk 1322/1468 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part27_v4rho2sigma2_10_chunk1322<F: Float>(t34502: F, t1: F, t106: F, t4524: F, t544: F, t191: F, t4529: F, t34378: F, t10517: F, t7014: F, t10615: F, t31167: F) -> (F, F, F, F) {
    let t34503 = F::new(0.89376224879626066674e-1) * t34502;
    let t34506 = t544 * t4524 * t1 * t106;
    let t34507 = t191 * t4529;
    let t34510 = F::new(0.85801175884441024004e1) * t34506 * t34507 * t34378;
    let t34512 = F::new(0.87421871174939309262e2) * t7014 * t10517;
    let t34530 = t10615 * t31167;
    (t34503, t34510, t34512, t34530)
}
