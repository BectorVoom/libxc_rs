//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 1260/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk1260<F: Float>(t1020: F, t1129: F, t1131: F, t12285: F, t12298: F, t12300: F, t12302: F, t12894: F, t2410: F, t2956: F, t3522: F, t3524: F, t3526: F, t3530: F, t3745: F, t3749: F, t3753: F, t3757: F, t839: F, t9707: F) -> F {
    let t44778 = -F::cast_from(0.18428227254588e2_f64) * t12298 * t1020 - F::cast_from(0.18428227254588e2_f64) * t3749 * t2410 - F::cast_from(0.8704e0_f64) * t2956 * t3522 - F::cast_from(0.17408e1_f64) * t2410 * t3745 - F::cast_from(0.17408e1_f64) * t1020 * t12285 - F::cast_from(0.8704e0_f64) * t839 * t12894 - F::cast_from(0.9214113627294e1_f64) * t3524 * t2956 - F::cast_from(0.9214113627294e1_f64) * t3526 * t2956 - F::cast_from(0.9214113627294e1_f64) * t1129 * t9707 + F::cast_from(0.734774460522e2_f64) * t12300 * t1020 + F::cast_from(0.734774460522e2_f64) * t3753 * t2410 + F::cast_from(0.367387230261e2_f64) * t3530 * t2956 + F::cast_from(0.367387230261e2_f64) * t1131 * t9707 - F::cast_from(0.7662840944824e2_f64) * t12302 * t1020 - F::cast_from(0.7662840944824e2_f64) * t3757 * t2410;
    t44778
}
