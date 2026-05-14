//! GGA_C_GAPLOC lxc pol — lxc_pol part 48 (v4rhosigma3_13) CSE chunk 745/861 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part48_v4rhosigma3_13_chunk745<F: Float>(t13343: F, t17288: F, t13483: F, t1377: F, t10305: F, t8045: F, t13350: F, t4349: F, t605: F, t1382: F, t2497: F, t3599: F, t11298: F, t19933: F, t11556: F, t921: F) -> (F, F, F, F, F, F, F) {
    let t44678 = 6.0 * t17288 * t13343;
    let t44679 = t1377 * t13483;
    let t44681 = 4.0 * t8045 * t10305;
    let t44684 = 6.0 * t4349 * t13350 * t605;
    let t44687 = 2.0 * t1382 * t3599 * t2497;
    let t44689 = 6.0 * t19933 * t11298;
    let t44692 = 2.0 * t1382 * t11556 * t921;
    (t44678, t44679, t44681, t44684, t44687, t44689, t44692)
}
