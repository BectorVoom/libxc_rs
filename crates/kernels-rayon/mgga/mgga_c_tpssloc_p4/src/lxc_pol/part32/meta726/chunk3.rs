//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 2344/2369 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2344(t103125: f64, t104721: f64, t113: f64, t1393: f64, t1849: f64, t19289: f64, t19450: f64, t20098: f64, t2114: f64, t2165: f64, t2167: f64, t27903: f64, t29497: f64, t33690: f64, t4073: f64, t96355: f64, t96358: f64, t96360: f64, t96738: f64, t96740: f64, t96746: f64, t96755: f64, t96758: f64, t96760: f64, t96763: f64, t96765: f64, t96767: f64) -> f64 {
    let t104727 = t96355 - t96358 - t96360 + t29497 * t1393 - t2114 * t19289 - t96738 - t96740 + 2.0_f64 * t27903 * t1849 + t96746 - t96755 - t96758 + t96760 + t2167 * t20098 - t113 * (t103125 + t104721) - t19450 * t2165 + t96763 - t96765 - t96767 - 4.0_f64 * t33690 * t4073;
    t104727
}
