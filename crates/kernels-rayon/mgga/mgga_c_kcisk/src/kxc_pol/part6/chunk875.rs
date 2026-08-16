//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 875/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk875(t1842: f64, t28385: f64, t1659: f64, t28377: f64, t11524: f64, t11528: f64, t11532: f64, t16204: f64, t16206: f64, t16208: f64, t16225: f64, t16227: f64, t165: f64, t173: f64, t28610: f64) -> f64 {
    let t28613 = t1842 * t28385;
    let t28616 = t1659 * t28377;
    let t28619 = 0.10566666666666666666e-1_f64 * t16204 + 0.117630625e-3_f64 * t16206 - 0.32788e-1_f64 * t16208 + 0.71734315950379065738e-1_f64 * t16225 - 0.93231700340333523768e-3_f64 * t16227 - t11524 + t11528 + t11532 - 0.30247875e-4_f64 * t173 * t28610 - 0.4755e-2_f64 * t165 * t28613 - 0.1585e-2_f64 * t165 * t28616;
    t28619
}
