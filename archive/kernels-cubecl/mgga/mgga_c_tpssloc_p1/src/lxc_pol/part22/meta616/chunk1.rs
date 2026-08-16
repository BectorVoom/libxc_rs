//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2146/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2146<F: Float>(t1174: F, t1709: F, t44633: F, t11530: F, t4889: F, t50853: F, t51039: F, t51051: F, t457: F, t4936: F, t698: F, t11529: F, t4912: F) -> (F, F, F, F, F, F, F) {
    let t52281 = t1174 * t44633 * t1709;
    let t52288 = t4889 * t11530;
    let t52313 = F::cast_from(5.0_f64) / F::cast_from(9.0_f64) * t50853;
    let t52339 = F::cast_from(10.0_f64) / F::cast_from(9.0_f64) * t51039;
    let t52343 = F::cast_from(5.0_f64) / F::cast_from(27.0_f64) * t51051;
    let t52354 = t1174 * t698 * t457 * t4936;
    let t52355 = F::cast_from(0.55555555555555555554e-3_f64) * t52354;
    let t52367 = t1174 * t11529 * t4912;
    (t52281, t52288, t52313, t52339, t52343, t52355, t52367)
}
