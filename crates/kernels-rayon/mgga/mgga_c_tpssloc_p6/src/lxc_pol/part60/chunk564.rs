//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 60 (v4rho2sigma2_16) CSE chunk 564/1064 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part60_v4rho2sigma2_16_chunk564(t6528: f64, t6548: f64, t6564: f64, t6579: f64, t6586: f64, t6602: f64, t6617: f64, t2048: f64, t225: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t7053 = 2.0_f64 / 3.0_f64 * t6528;
    let t7067 = 0.38381794893125283518e-1_f64 * t6548;
    let t7069 = 0.82246703342411321825e-2_f64 * t6564;
    let t7074 = 7.0_f64 / 144.0_f64 * t6579;
    let t7076 = 0.28260929265898273597e-2_f64 * t6586;
    let t7078 = 0.67287926823567318088e-4_f64 * t6602;
    let t7082 = 7.0_f64 / 1152.0_f64 * t6617;
    let t7087 = t2048 * t225;
    (t7053, t7067, t7069, t7074, t7076, t7078, t7082, t7087)
}
