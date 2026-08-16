//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1363/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk1363<F: Float>(t11061: F, t7788: F, t8090: F, t26972: F, t8083: F, t2197: F, t26960: F, t27083: F, t5345: F, t7772: F, t92941: F, t92943: F, t92946: F, t92951: F, t92955: F, t92958: F, t96968: F, t97039: F) -> F {
    let t97153 = t7788 * t11061 * t8090;
    let t97166 = t8083 * t26972;
    let t97170 = F::cast_from(0.25742669753086419753e-4_f64) * t97153 - F::cast_from(0.2782641015625e-3_f64) * t7772 * t97039 + F::cast_from(0.20594135802469135802e-3_f64) * t92941 - F::cast_from(0.15476481481481481481e-2_f64) * t92943 + F::cast_from(0.11584201388888888889e-3_f64) * t92946 - F::cast_from(0.82448622685185185185e-4_f64) * t92951 + F::cast_from(0.23168402777777777778e-3_f64) * t92955 + F::cast_from(0.23168402777777777778e-3_f64) * t92958 - F::cast_from(0.33980324074074074074e-2_f64) * t5345 * t27083 * t2197 + F::cast_from(0.7722800925925925926e-4_f64) * t97166 - F::cast_from(0.23168402777777777778e-3_f64) * t26960 * t96968;
    t97170
}
