//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 1966/2357 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1966(t27732: f64, t462: f64, t1170: f64, t8077: f64, t2121: f64, t1201: f64, t1244: f64, t1729: f64, t2152: f64, t24856: f64, t27572: f64, t27574: f64, t27722: f64, t27725: f64, t27728: f64, t470: f64, t4964: f64, t7283: f64, t7382: f64, t7389: f64, t7999: f64, t8085: f64) -> f64 {
    let t27733 = t462 * t27732;
    let t27736 = t1170 * t8077;
    let t27737 = t2121 * t27736;
    let t27739 = t1729 * t7389 + t4964 * t2152 + t1201 * t8085 - 0.73108180748810063843e-2_f64 * t27572 - 0.82246703342411321825e-2_f64 * t7283 * t27574 + t470 * t27722 + t1244 * t27725 - 0.91385225936012579807e-3_f64 * t24856 - 0.27415567780803773942e-2_f64 * t27728 - 0.21932454224643019153e-1_f64 * t7999 * t7382 + 0.82246703342411321825e-2_f64 * t2121 * t27733 + 0.27415567780803773942e-2_f64 * t27737;
    t27739
}
