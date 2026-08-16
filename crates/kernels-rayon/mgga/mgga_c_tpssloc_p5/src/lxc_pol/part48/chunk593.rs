//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 48 (v4rho2sigma2_4) CSE chunk 593/1034 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part48_v4rho2sigma2_4_chunk593(t2075: f64, t671: f64, t6548: f64, t6564: f64, t2047: f64, t798: f64, t6579: f64, t6586: f64, t6602: f64, t6617: f64, t6582: f64, t6594: f64, t6607: f64, t6610: f64, t6615: f64, t6622: f64) -> (f64, f64, f64, f64, f64) {
    let t7061 = t2075 * t671;
    let t7067 = 0.38381794893125283518e-1_f64 * t6548;
    let t7069 = 0.82246703342411321825e-2_f64 * t6564;
    let t7072 = t798 * t2047;
    let t7074 = 7.0_f64 / 144.0_f64 * t6579;
    let t7076 = 0.28260929265898273597e-2_f64 * t6586;
    let t7078 = 0.67287926823567318088e-4_f64 * t6602;
    let t7082 = 7.0_f64 / 1152.0_f64 * t6617;
    let t7084 = -t7074 - t6582 / 24.0_f64 - t7076 - 0.24223653656484234512e-2_f64 * t6594 - t7078 - 0.40372756094140390853e-3_f64 * t6607 + t6610 / 768.0_f64 - t6615 / 768.0_f64 - t7082 - t6622 / 192.0_f64;
    (t7061, t7067, t7069, t7072, t7084)
}
