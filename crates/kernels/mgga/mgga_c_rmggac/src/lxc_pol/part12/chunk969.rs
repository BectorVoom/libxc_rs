//! MGGA_C_RMGGAC lxc pol — lxc_pol part 12 (v4rho3sigma_3) CSE chunk 969/1088 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part12_v4rho3sigma_3_chunk969<F: Float>(t40564: F, t2320: F, t35151: F, t34847: F, t8668: F, t1525: F, t236: F, t498: F, t7230: F, t7231: F, t333: F, t8957: F) -> (F, F, F, F, F) {
    let t40565 = F::new(0.24829349937757072982e-4) * t40564;
    let t40566 = t35151 * t2320;
    let t40567 = F::new(0.24829349937757072982e-4) * t40566;
    let t40568 = t34847 * t8668;
    let t40573 = t7230 * t7231 * t236 * t1525 * t498;
    let t40575 = t8957 * t333;
    (t40565, t40567, t40568, t40573, t40575)
}
