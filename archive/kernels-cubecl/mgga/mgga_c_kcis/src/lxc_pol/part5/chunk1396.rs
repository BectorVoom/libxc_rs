//! MGGA_C_KCIS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 1396/1419 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part5_v3rho3_2_chunk1396<F: Float>(t23006: F, t23023: F, t16663: F, t18037: F, t18040: F, t18041: F, t21848: F, t21852: F, t21856: F, t21861: F, t21865: F, t21872: F, t21874: F, t21879: F, t21881: F, t21884: F, t21889: F, t21896: F, t21900: F, t21903: F, t21908: F, t626: F) -> (F, F) {
    let t23024 = t23006 + t23023;
    let t23030 = -F::cast_from(0.15476481481481481481e-2_f64) * t21848 - F::cast_from(0.15476481481481481481e-2_f64) * t21852 - F::cast_from(0.51588271604938271603e-3_f64) * t21856 - F::cast_from(0.30952962962962962962e-2_f64) * t21861 + F::cast_from(0.25794135802469135802e-2_f64) * t21865 - F::cast_from(0.51588271604938271603e-3_f64) * t16663 + F::cast_from(0.19345601851851851852e-2_f64) * t21872 + F::cast_from(0.10317654320987654321e-2_f64) * t21874 + F::cast_from(0.69644166666666666664e-2_f64) * t21879 - F::cast_from(0.15476481481481481481e-2_f64) * t21881 - F::cast_from(0.61905925925925925925e-2_f64) * t21884 + F::cast_from(0.11349419753086419753e-1_f64) * t21889 + t23024 * t626 - t18037 + t18040 + t18041 + F::cast_from(0.92858888888888888888e-2_f64) * t21896 - F::cast_from(0.25794135802469135802e-3_f64) * t21900 + F::cast_from(0.17024129629629629629e-1_f64) * t21903 + F::cast_from(0.38691203703703703703e-3_f64) * t21908;
    (t23024, t23030)
}
