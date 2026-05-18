//! GGA_C_GAPLOC lxc pol — lxc_pol part 42 (v4rhosigma3_7) CSE chunk 830/1012 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part42_v4rhosigma3_7_chunk830<F: Float>(t10295: F, t27214: F, t3073: F, t3459: F, t5559: F, t35781: F, t977: F, t11288: F, t2497: F, t3601: F, t935: F) -> (F, F, F, F, F) {
    let t44694 = F::new(12.0) * t27214 * t10295;
    let t44697 = F::new(12.0) * t5559 * t3459 * t3073;
    let t44702 = t35781 * t977;
    let t44705 = t11288 * t2497;
    let t44707 = t3601 * t935;
    (t44694, t44697, t44702, t44705, t44707)
}
