//! MGGA_C_R2SCAN lxc pol — lxc_pol part 13 (v4rho3sigma_3) CSE chunk 912/1253 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part13_v4rho3sigma_3_chunk912<F: Float>(t1020: F, t1310: F, t2410: F, t839: F, t333: F, t8438: F, t335: F, t337: F, t339: F, t341: F, t1028: F, t1030: F, t2426: F, t2430: F, t343: F) -> (F, F, F) {
    let t8463 = t1310 * t1020;
    let t8465 = t839 * t2410;
    let t8467 = t333 * t8438;
    let t8469 = t335 * t8438;
    let t8471 = t337 * t8438;
    let t8473 = t339 * t8438;
    let t8475 = t341 * t8438;
    let t8479 = F::cast_from(0.3101306810232e2_f64) * t2426 * t839 + F::cast_from(0.1550653405116e2_f64) * t1028 * t1310 - F::cast_from(0.4355305902528e1_f64) * t2430 * t839 - F::cast_from(0.2177652951264e1_f64) * t1030 * t1310 - F::cast_from(0.8704e0_f64) * t8463 - F::cast_from(0.17408e1_f64) * t8465 - F::cast_from(0.8704e0_f64) * t8467 - F::cast_from(0.4607056813647e1_f64) * t8469 + F::cast_from(0.122462410087e2_f64) * t8471 - F::cast_from(0.957855118103e1_f64) * t8473 + F::cast_from(0.3101306810232e1_f64) * t8475 - F::cast_from(0.362942158544e0_f64) * t343 * t8438;
    (t8463, t8465, t8479)
}
