//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 2137/2369 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2137(t26179: f64, t7468: f64, t26003: f64, t7458: f64, t26142: f64, t4028: f64, t22674: f64, t28191: f64, t80681: f64, t1985: f64, t22666: f64, t28232: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t96839 = 4.0_f64 * t26179 * t7468;
    let t96842 = 4.0_f64 * t7458 * t26003;
    let t96844 = 4.0_f64 * t7458 * t26142;
    let t96846 = 4.0_f64 * t4028 * t26142;
    let t96848 = t80681 * t22674 * t28191;
    let t96851 = t1985 * t22666 * t28232;
    (t96839, t96842, t96844, t96846, t96848, t96851)
}
