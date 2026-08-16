//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 619/1322 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk619<F: Float>(t3210: F, t6625: F, t3200: F, t1022: F, t6330: F, t1021: F, t1020: F, t1710: F, t4787: F, t4981: F, t5003: F, t5017: F, t5023: F, t6558: F, t6561: F, t6564: F, t6616: F, t6622: F) -> (F, F, F, F, F, F) {
    let t6626 = t3210 * t6625;
    let t6627 = t3200 * t6626;
    let t6629 = t1022 * t6330;
    let t6630 = t1021 * t6629;
    let t6631 = t1020 * t6630;
    let t6633 = -F::cast_from(0.13345e0_f64) * t4981 * t1710 - F::cast_from(0.33163888888888888888e-2_f64) * t5017 + F::cast_from(0.22109259259259259258e-2_f64) * t5023 + F::cast_from(0.33163888888888888888e-2_f64) * t4787 + F::cast_from(0.16581944444444444444e-2_f64) * t6558 - F::cast_from(0.49745833333333333332e-2_f64) * t6561 + F::cast_from(0.33163888888888888888e-2_f64) * t6564 - F::cast_from(0.24872916666666666666e-2_f64) * t6616 + F::cast_from(0.22109259259259259258e-2_f64) * t5003 - F::cast_from(0.33163888888888888888e-2_f64) * t6622 + F::cast_from(0.22109259259259259258e-2_f64) * t6627 - F::cast_from(0.33163888888888888888e-2_f64) * t6631;
    (t6626, t6627, t6629, t6630, t6631, t6633)
}
