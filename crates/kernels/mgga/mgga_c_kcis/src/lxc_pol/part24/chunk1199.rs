//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1199/1322 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk1199<F: Float>(t3530: F, t417: F, t1851: F, t3622: F, t1268: F, t9372: F, t11081: F, t26960: F, t28097: F, t95571: F, t27014: F, t28093: F) -> (F, F, F, F, F, F) {
    let t96742 = t417 * t3530;
    let t96743 = t3622 * t1851;
    let t96754 = t1268 * t9372;
    let t96763 = F::cast_from(0.7722800925925925926e-4_f64) * t26960 * t11081 * t28097;
    let t96779 = F::cast_from(0.25794135802469135802e-2_f64) * t95571;
    let t96781 = F::cast_from(0.23168402777777777778e-3_f64) * t27014 * t28093;
    (t96742, t96743, t96754, t96763, t96779, t96781)
}
