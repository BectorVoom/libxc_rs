//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2928/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2928(t1068: f64, t4696: f64, t13508: f64, t4483: f64, t17934: f64, t2948: f64, t13718: f64, t10723: f64, t17954: f64, t959: f64, t17937: f64, t2925: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t60941 = t4696 * t1068;
    let t60946 = 0.34631718211362927517e2_f64 * t4483 * t13508;
    let t60953 = 0.5848223622634646207e0_f64 * t17934 * t2948;
    let t60955 = 0.11696447245269292414e1_f64 * t4483 * t13718;
    let t60958 = 0.17315859105681463759e2_f64 * t959 * t17954 * t10723;
    let t60961 = 0.11696447245269292414e1_f64 * t959 * t17937 * t2925;
    (t60941, t60946, t60953, t60955, t60958, t60961)
}
