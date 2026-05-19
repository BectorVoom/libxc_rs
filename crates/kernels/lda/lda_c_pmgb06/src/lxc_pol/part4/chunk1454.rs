//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1454/1478 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1454<F: Float>(t8279: F, t11307: F, t11310: F, t11317: F, t11304: F, t11313: F, t8266: F, t8282: F, t8285: F, t8287: F, t8291: F, t8293: F, t8295: F) -> (F, F, F, F, F) {
    let t18634 = F::cast_from(3.8973666666666666_f64) * t8279;
    let t18637 = F::cast_from(3.8973666666666666_f64) * t11307;
    let t18638 = F::cast_from(2.5982444444444446_f64) * t11310;
    let t18640 = F::cast_from(5.196488888888889_f64) * t11317;
    let t18641 = -t8266 + t18634 - F::new(1.95872) * t8282 + t8285 + t8287 + t8291 + t8293 - t8295 + F::new(7.83488) * t11304 - t18637 - t18638 - F::new(2.0) * t11313 + t18640;
    (t18634, t18637, t18638, t18640, t18641)
}
