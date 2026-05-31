//! GGA_C_ACGGAP lxc pol — lxc_pol part 11 (v4rho3sigma_3) CSE chunk 1032/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part11_v4rho3sigma_3_chunk1032<F: Float>(t1165: F, t4550: F, t7351: F, t7575: F, t1530: F, t1535: F, t30539: F, t4762: F, t7564: F, t8600: F, t30308: F, t30310: F) -> (F, F, F, F, F) {
    let t34201 = t7575 * t1165 * t7351 * t4550;
    let t34204 = t1530 * t30539 * t1535;
    let t34208 = t7564 * t1165 * t8600 * t4762;
    let t34210 = F::cast_from(77.0_f64) / F::cast_from(288.0_f64) * t30308;
    let t34211 = F::cast_from(77.0_f64) / F::cast_from(864.0_f64) * t30310;
    (t34201, t34204, t34208, t34210, t34211)
}
