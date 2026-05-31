//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 522/1276 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk522<F: Float>(t1020: F, t839: F, t2410: F, t333: F, t335: F, t337: F, t339: F, t341: F, t1022: F, t1024: F, t1026: F, t1028: F, t1030: F, t343: F) -> (F, F, F, F, F, F, F) {
    let t2412 = t839 * t1020;
    let t2414 = t333 * t2410;
    let t2418 = t335 * t2410;
    let t2422 = t337 * t2410;
    let t2426 = t339 * t2410;
    let t2430 = t341 * t2410;
    let t2436 = -F::cast_from(0.64e0_f64) * t2410 - F::cast_from(0.8704e0_f64) * t2412 - F::cast_from(0.8704e0_f64) * t2414 - F::cast_from(0.9214113627294e1_f64) * t1022 * t839 - F::cast_from(0.4607056813647e1_f64) * t2418 + F::cast_from(0.367387230261e2_f64) * t1024 * t839 + F::cast_from(0.122462410087e2_f64) * t2422 - F::cast_from(0.3831420472412e2_f64) * t1026 * t839 - F::cast_from(0.957855118103e1_f64) * t2426 + F::cast_from(0.1550653405116e2_f64) * t1028 * t839 + F::cast_from(0.3101306810232e1_f64) * t2430 - F::cast_from(0.2177652951264e1_f64) * t1030 * t839 - F::cast_from(0.362942158544e0_f64) * t343 * t2410;
    (t2412, t2414, t2418, t2422, t2426, t2430, t2436)
}
