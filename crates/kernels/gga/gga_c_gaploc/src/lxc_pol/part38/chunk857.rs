//! GGA_C_GAPLOC lxc pol — lxc_pol part 38 (v4rhosigma3_3) CSE chunk 857/1003 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part38_v4rhosigma3_3_chunk857<F: Float>(t11298: F, t19933: F, t11556: F, t1382: F, t921: F, t10295: F, t27214: F, t3073: F, t3459: F, t5559: F, t35781: F, t977: F) -> (F, F, F, F, F) {
    let t44689 = F::new(6.0) * t19933 * t11298;
    let t44692 = F::new(2.0) * t1382 * t11556 * t921;
    let t44694 = F::new(12.0) * t27214 * t10295;
    let t44697 = F::new(12.0) * t5559 * t3459 * t3073;
    let t44702 = t35781 * t977;
    (t44689, t44692, t44694, t44697, t44702)
}
