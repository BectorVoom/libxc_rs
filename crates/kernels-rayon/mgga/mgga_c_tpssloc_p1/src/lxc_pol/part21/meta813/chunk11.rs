//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2868/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2868(t10813: f64, t5758: f64, t17195: f64, t2837: f64, t2841: f64, t5689: f64, t2845: f64, t17471: f64, t923: f64, t1557: f64, t49483: f64, t13515: f64, t4396: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t59941 = t5758 * t10813;
    let t59958 = 1.0_f64 * t17195 * t2837;
    let t59959 = t5689 * t2841;
    let t59961 = 0.16081979498692535067e2_f64 * t59959 * t2845;
    let t59962 = t17471 * t923;
    let t59966 = 2.0_f64 * t49483 * t1557;
    let t59968 = 4.0_f64 * t13515 * t4396;
    (t59941, t59958, t59961, t59962, t59966, t59968)
}
