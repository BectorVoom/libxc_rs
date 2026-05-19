//! MGGA_C_R2SCAN lxc pol — lxc_pol part 15 (v4rho3sigma_5) CSE chunk 1244/1253 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part15_v4rho3sigma_5_chunk1244<F: Float>(t1020: F, t1081: F, t1087: F, t11110: F, t11115: F, t11930: F, t1310: F, t1312: F, t2410: F, t333: F, t335: F, t337: F, t3386: F, t339: F, t3398: F, t341: F, t343: F, t3648: F, t3656: F, t40893: F, t839: F, t8438: F) -> F {
    let t41019 = F::cast_from(0.367387230261e2_f64) * t3656 * t1310 - F::cast_from(0.3831420472412e2_f64) * t11115 * t1020 - F::cast_from(0.7662840944824e2_f64) * t3398 * t2410 - F::cast_from(0.3831420472412e2_f64) * t1087 * t8438 - F::cast_from(0.362942158544e0_f64) * t343 * t40893 - F::new(0.8704e0) * t8438 * t1081 - F::new(0.17408e1) * t2410 * t3386 - F::new(0.8704e0) * t1020 * t11110 - F::new(0.8704e0) * t1310 * t3648 - F::new(0.17408e1) * t839 * t11930 - F::new(0.8704e0) * t333 * t40893 - F::cast_from(0.4607056813647e1_f64) * t335 * t40893 + F::cast_from(0.122462410087e2_f64) * t337 * t40893 - F::cast_from(0.957855118103e1_f64) * t339 * t40893 + F::cast_from(0.3101306810232e1_f64) * t341 * t40893 - F::cast_from(0.9214113627294e1_f64) * t1312 * t3648;
    t41019
}
