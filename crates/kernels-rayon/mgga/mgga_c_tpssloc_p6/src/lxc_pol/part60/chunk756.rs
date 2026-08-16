//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 60 (v4rho2sigma2_16) CSE chunk 756/1064 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part60_v4rho2sigma2_16_chunk756(t27683: f64, t7324: f64, t1210: f64, t8039: f64, t24721: f64, t6739: f64, t8026: f64, t7325: f64, t24574: f64, t8070: f64, t1170: f64, t8077: f64) -> (f64, f64, f64, f64, f64) {
    let t27684 = t7324 * t27683;
    let t27700 = t1210 * t8039;
    let t27701 = t24721 * t27700;
    let t27710 = t8026 * t6739;
    let t27711 = t27710 * t7325;
    let t27728 = t24574 * t8070;
    let t27736 = t1170 * t8077;
    (t27684, t27701, t27711, t27728, t27736)
}
