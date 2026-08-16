//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 907/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk907(t1707: f64, t29138: f64, t11091: f64, t11092: f64, t17382: f64, t17385: f64, t23472: f64, t23481: f64, t23570: f64, t29088: f64, t29094: f64, t29116: f64, t29121: f64, t29124: f64, t29126: f64) -> (f64, f64) {
    let t29139 = t1707 * t29138;
    let t29146 = -0.16557e0_f64 * t29116 - 0.40256666666666666668e0_f64 * t17382 - 0.5519e0_f64 * t17385 + 0.99342e0_f64 * t29121 + 0.19419375e1_f64 * t29124 - t11091 - t11092 - 0.412621875e-1_f64 * t29126 + 0.258925e1_f64 * t29139 - 0.66228e0_f64 * t23570 - 0.60385000000000000001e0_f64 * t23472 + 0.30192500000000000001e0_f64 * t23481 - 0.60384999999999999999e0_f64 * t29088 + 0.181155e1_f64 * t29094;
    (t29139, t29146)
}
