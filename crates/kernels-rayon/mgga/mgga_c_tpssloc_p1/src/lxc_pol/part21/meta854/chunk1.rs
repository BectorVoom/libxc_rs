//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 3088/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3088(t43855: f64, t43859: f64, t43861: f64, t43863: f64, t44027: f64, t50903: f64, t50905: f64, t50907: f64, t50919: f64, t50921: f64, t50948: f64, t50950: f64, t50952: f64, t50954: f64) -> f64 {
    let t64027 = t44027 - 0.30428395061728395062e-1_f64 * t43855 - 0.486854320987654321e0_f64 * t43859 + 0.91285185185185185187e-1_f64 * t43861 + 0.18257037037037037037e0_f64 * t43863 - 0.79724444444444444444e0_f64 * t50903 - 0.39862222222222222222e0_f64 * t50905 - 0.11958666666666666667e1_f64 * t50907 - 0.35433086419753086419e0_f64 * t50919 - 0.22145679012345679012e0_f64 * t50921 + 0.10629925925925925926e1_f64 * t50948 + 0.26574814814814814814e0_f64 * t50950 + 0.13287407407407407407e0_f64 * t50952 + 0.79724444444444444443e0_f64 * t50954;
    t64027
}
