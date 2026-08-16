//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2554/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2554(t50948: f64, t43780: f64, t43782: f64, t43784: f64, t43786: f64, t43788: f64, t43816: f64, t44320: f64, t50937: f64, t50940: f64, t50946: f64, t50950: f64, t50952: f64, t50954: f64, t50957: f64, t50961: f64, t50966: f64, t50994: f64, t51000: f64, t51004: f64) -> f64 {
    let t51769 = 0.4566222222222222222e-1_f64 * t50948;
    let t51785 = 0.30822e0_f64 * t50937 + 0.34246666666666666666e-1_f64 * t50940 + 0.41096e0_f64 * t50946 + t51769 + 0.2283111111111111111e-1_f64 * t50950 + 0.11415555555555555555e-1_f64 * t50952 + 0.6849333333333333333e-1_f64 * t50954 - 0.34246666666666666665e-1_f64 * t50957 - 0.34246666666666666665e-1_f64 * t50961 - 0.20547999999999999999e0_f64 * t50966 + t44320 + 0.2283111111111111111e-1_f64 * t43780 + 0.4566222222222222222e-1_f64 * t43782 + 0.2283111111111111111e-1_f64 * t43784 - 0.34246666666666666665e-1_f64 * t43786 - 0.57077777777777777777e-2_f64 * t43788 - 0.53272592592592592591e-1_f64 * t43816 - 0.20547999999999999999e0_f64 * t50994 + 0.30822e0_f64 * t51000 + 0.57077777777777777775e-1_f64 * t51004;
    t51785
}
