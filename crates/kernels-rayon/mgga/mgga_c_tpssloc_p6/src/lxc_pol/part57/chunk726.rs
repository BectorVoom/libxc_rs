//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 57 (v4rho2sigma2_13) CSE chunk 726/1049 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part57_v4rho2sigma2_13_chunk726(t23164: f64, t25345: f64, t225: f64, t7511: f64, t2752: f64, t7540: f64, t10143: f64, t25: f64, t1625: f64, t6703: f64, t381: f64, t7577: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t25346 = t23164 * t25345;
    let t25348 = t7511 * t225;
    let t25358 = t7540 * t2752;
    let t25373 = t10143 * t25;
    let t25406 = t6703 * t1625;
    let t25442 = t7577 * t381;
    (t25346, t25348, t25358, t25373, t25406, t25442)
}
