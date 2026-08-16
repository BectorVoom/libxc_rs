//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2518/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2518(t50948: f64, t43780: f64, t43782: f64, t43784: f64, t43786: f64, t43788: f64, t43816: f64, t43820: f64, t50937: f64, t50940: f64, t50946: f64, t50950: f64, t50952: f64, t50954: f64, t50957: f64, t50961: f64, t50966: f64, t50994: f64, t51000: f64, t51004: f64) -> f64 {
    let t51082 = 8.0_f64 / 9.0_f64 * t50948;
    let t51098 = 6.0_f64 * t50937 + 2.0_f64 / 3.0_f64 * t50940 + 8.0_f64 * t50946 + t51082 + 4.0_f64 / 9.0_f64 * t50950 + 2.0_f64 / 9.0_f64 * t50952 + 4.0_f64 / 3.0_f64 * t50954 - 2.0_f64 / 3.0_f64 * t50957 - 2.0_f64 / 3.0_f64 * t50961 - 4.0_f64 * t50966 + t43820 + 4.0_f64 / 9.0_f64 * t43780 + 8.0_f64 / 9.0_f64 * t43782 + 4.0_f64 / 9.0_f64 * t43784 - 2.0_f64 / 3.0_f64 * t43786 - t43788 / 9.0_f64 - 28.0_f64 / 27.0_f64 * t43816 - 4.0_f64 * t50994 + 6.0_f64 * t51000 + 10.0_f64 / 9.0_f64 * t51004;
    t51098
}
