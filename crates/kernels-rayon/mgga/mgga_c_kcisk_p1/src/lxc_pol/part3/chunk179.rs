//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 179/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk179(t429: f64, t430: f64, t435: f64, t436: f64, t445: f64, t446: f64, t674: f64, t686: f64, t690: f64, t696: f64, t698: f64) -> f64 {
    let t702 = -0.11955719325063177623e-1_f64 * t674 + 0.263475e-2_f64 * t429 * t430 * t686 - 0.4755e-3_f64 * t435 * t436 * t690 + 0.2589769453898153438e-4_f64 * t696 - 0.21605625e-5_f64 * t445 * t446 * t698;
    t702
}
