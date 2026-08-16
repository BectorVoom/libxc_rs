//! GGA_C_GAPLOC lxc pol — lxc_pol part 30 (v4rho2sigma2_13) CSE chunk 1358/1436 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part30_v4rho2sigma2_13_chunk1358<F: Float>(t2877: F, t30789: F, t30703: F, t10597: F, t1537: F, t30297: F, t30299: F, t30305: F, t10600: F, t1415: F, t20902: F, t31585: F, t493: F) -> (F, F, F, F, F, F, F, F) {
    let t34256 = F::cast_from(0.35750489951850426669e0_f64) * t30789 * t2877;
    let t34258 = F::cast_from(0.71500979903700853338e0_f64) * t30703 * t2877;
    let t34259 = t1537 * t10597;
    let t34260 = F::cast_from(0.25561950635947166451e1_f64) * t34259;
    let t34261 = F::cast_from(0.15976219147466979032e-1_f64) * t30297;
    let t34262 = F::cast_from(0.31952438294933958064e-1_f64) * t30299;
    let t34263 = F::cast_from(0.63904876589867916128e-1_f64) * t30305;
    let t34264 = t1415 * t10600;
    let t34266 = F::cast_from(0.79445533226334281486e-1_f64) * t34264 * t20902;
    let t34267 = t493 * t31585;
    (t34256, t34258, t34260, t34261, t34262, t34263, t34266, t34267)
}
