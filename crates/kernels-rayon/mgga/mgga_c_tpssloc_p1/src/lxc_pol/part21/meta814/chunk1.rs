//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2870/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2870(t4359: f64, t49486: f64, t4400: f64, t49269: f64, t13727: f64, t14379: f64, t10661: f64, t2793: f64, t5695: f64, t13520: f64, t14389: f64, t10655: f64, t17507: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t60006 = 8.0_f64 * t49486 * t4359;
    let t60008 = 0.64327917994770140268e2_f64 * t49269 * t4400;
    let t60010 = 8.0_f64 * t13727 * t14379;
    let t60016 = 24.0_f64 * t10661 * t5695 * t2793;
    let t60021 = 0.64327917994770140268e2_f64 * t13520 * t14389;
    let t60023 = 12.0_f64 * t10655 * t17507;
    (t60006, t60008, t60010, t60016, t60021, t60023)
}
