//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 40 (v4rho3tau_4) CSE chunk 1006/1303 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part40_v4rho3tau_4_chunk1006<F: Float>(t11588: F, t1714: F, t3451: F, t3447: F, t14818: F, t14781: F, t14710: F, t1716: F, t698: F, t1174: F, t3435: F, t4889: F) -> (F, F, F, F, F, F, F) {
    let t15338 = t11588 * t1714;
    let t15339 = t15338 * t3451;
    let t15341 = F::cast_from(0.18518518518518518518e-3_f64) * t3447 * t15339;
    let t15347 = F::cast_from(2.0_f64) / F::cast_from(27.0_f64) * t14818;
    let t15348 = F::cast_from(4.0_f64) / F::cast_from(9.0_f64) * t14781;
    let t15349 = F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t14710;
    let t15363 = t698 * t1716;
    let t15364 = t1174 * t15363;
    let t15366 = t4889 * t3435;
    (t15338, t15341, t15347, t15348, t15349, t15364, t15366)
}
