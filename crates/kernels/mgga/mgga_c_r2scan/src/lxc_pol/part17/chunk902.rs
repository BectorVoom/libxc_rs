//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 902/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk902<F: Float>(t1020: F, t2410: F, t2956: F, t839: F, t333: F, t9707: F, t2958: F, t335: F, t337: F, t1022: F, t1024: F, t2960: F, t2962: F, t2964: F) -> (F, F, F, F) {
    let t9709 = t1020 * t2410;
    let t9711 = t839 * t2956;
    let t9713 = t333 * t9707;
    let t9715 = t839 * t2958;
    let t9721 = t335 * t9707;
    let t9729 = t337 * t9707;
    let t9731 = -F::cast_from(0.64e0_f64) * t9707 - F::cast_from(0.17408e1_f64) * t9709 - F::cast_from(0.8704e0_f64) * t9711 - F::cast_from(0.8704e0_f64) * t9713 - F::cast_from(0.9214113627294e1_f64) * t9715 - F::cast_from(0.18428227254588e2_f64) * t1022 * t2410 - F::cast_from(0.9214113627294e1_f64) * t2960 * t839 - F::cast_from(0.4607056813647e1_f64) * t9721 + F::cast_from(0.734774460522e2_f64) * t2962 * t839 + F::cast_from(0.734774460522e2_f64) * t1024 * t2410 + F::cast_from(0.367387230261e2_f64) * t2964 * t839 + F::cast_from(0.122462410087e2_f64) * t9729;
    (t9709, t9711, t9715, t9731)
}
