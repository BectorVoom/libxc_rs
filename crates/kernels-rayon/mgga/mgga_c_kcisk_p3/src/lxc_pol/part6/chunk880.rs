//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 880/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk880(t28619: f64, t28642: f64, t28671: f64, t28694: f64, t16088: f64, t16090: f64, t1809: f64, t2399: f64, t28579: f64, t28582: f64, t28585: f64, t28588: f64, t28592: f64, t28595: f64, t28598: f64, t5089: f64, t604: f64, t674: f64, t8662: f64) -> (f64, f64) {
    let t28696 = t28619 + t28642 + t28671 + t28694;
    let t28698 = -0.28111840756657074597e-1_f64 * t5089 * t28579 + 0.14055920378328537299e-1_f64 * t5089 * t28582 + 0.14055920378328537299e-1_f64 * t1809 * t28585 + 0.14055920378328537299e-1_f64 * t1809 * t28588 - 0.56223681513314149196e-1_f64 * t674 * t28592 + 0.42167761134985611897e-1_f64 * t674 * t28595 - 0.42167761134985611897e-1_f64 * t1809 * t28598 - 0.14055920378328537299e-1_f64 * t16088 - 0.28111840756657074597e-1_f64 * t16090 - 3.0_f64 * t2399 * t8662 - t604 * t28696;
    (t28696, t28698)
}
