//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2686/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2686(t1388: f64, t5356: f64, t15899: f64, t39570: f64, t39585: f64, t39590: f64, t39593: f64, t5160: f64, t56391: f64, t56393: f64, t56395: f64, t56396: f64, t56398: f64, t56401: f64, t56403: f64) -> f64 {
    let t56404 = t1388 * t5356;
    let t56408 = 8.0_f64 * t15899 * t5160 * t56404 + t39570 - t39585 + t39590 - t39593 + t56391 - t56393 + t56395 + t56396 + t56398 + t56401 + t56403;
    t56408
}
