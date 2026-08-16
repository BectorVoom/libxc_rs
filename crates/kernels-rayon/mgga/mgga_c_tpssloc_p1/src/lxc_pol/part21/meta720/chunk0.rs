//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2564/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2564(t10482: f64, t3120: f64, t10470: f64, t11064: f64, t381: f64, t1057: f64, t49864: f64, t3199: f64, t49649: f64, t11045: f64, t14538: f64, t225: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t50510 = t10482 * t3120;
    let t50516 = t10470 * t11064 * t381;
    let t50535 = t49864 * t1057;
    let t50592 = t49649 * t3199;
    let t50610 = t10470 * t11045 * t381;
    let t50622 = t14538 * t225;
    (t50510, t50516, t50535, t50592, t50610, t50622)
}
