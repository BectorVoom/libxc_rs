//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 2111/2341 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2111(t12571: f64, t26083: f64, t1865: f64, t22523: f64, t22554: f64, t26051: f64, t26055: f64, t26067: f64, t26090: f64, t27976: f64, t6490: f64, t6492: f64, t7442: f64, t7446: f64, t96517: f64, t96521: f64, t96529: f64, t96532: f64, t96535: f64) -> f64 {
    let t96538 = t12571 * t26083;
    let t96545 = 5.0_f64 / 6.0_f64 * t22554 * t27976 + 5.0_f64 / 6.0_f64 * t22523 * t27976 + 5.0_f64 / 6.0_f64 * t6490 * t96517 + 5.0_f64 / 6.0_f64 * t6490 * t96521 + 5.0_f64 / 3.0_f64 * t26051 * t26067 + 2.0_f64 / 3.0_f64 * t26055 * t7446 + 5.0_f64 / 6.0_f64 * t96529 * t6492 + 5.0_f64 / 6.0_f64 * t96532 * t6492 + t96535 * t1865 / 3.0_f64 + 5.0_f64 / 3.0_f64 * t96538 * t6492 + 2.0_f64 / 3.0_f64 * t26055 * t7442 + 5.0_f64 / 3.0_f64 * t26051 * t26090;
    t96545
}
