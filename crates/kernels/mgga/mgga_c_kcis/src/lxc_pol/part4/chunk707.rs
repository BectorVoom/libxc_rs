//! MGGA_C_KCIS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 707/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part4_v3rho3_1_chunk707<F: Float>(t4108: F, t509: F, t552: F, t557: F, t303: F, t3245: F, t558: F, t1364: F, t1387: F, t3718: F, t3725: F, t3729: F, t3731: F, t3736: F, t3740: F, t3957: F, t3961: F, t3964: F, t4013: F) -> (F, F, F, F, F, F) {
    let t4109 = t509 * t4108;
    let t4110 = t4109 * t552;
    let t4111 = t4110 * t557;
    let t4112 = t303 * t4111;
    let t4114 = t3245 * t558;
    let t4115 = F::cast_from(0.55273148148148148147e-3_f64) * t4114;
    let t4116 = F::new(0.66725e-1) * t1364 * t3718 + F::cast_from(0.16581944444444444444e-2_f64) * t3725 - F::cast_from(0.33163888888888888888e-2_f64) * t3729 + F::cast_from(0.22109259259259259258e-2_f64) * t3731 - F::cast_from(0.49745833333333333332e-2_f64) * t3736 + F::cast_from(0.33163888888888888888e-2_f64) * t3740 - F::cast_from(0.24872916666666666666e-2_f64) * t3957 + F::cast_from(0.890445125e-2_f64) * t3961 * t3718 - F::new(0.13345e0) * t3964 * t1387 - F::new(0.66725e-1) * t1364 * t4013 + F::cast_from(0.24872916666666666666e-2_f64) * t4112 - t4115;
    (t4110, t4111, t4112, t4114, t4115, t4116)
}
