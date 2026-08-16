//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 3120/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3120(t1164: f64, t4883: f64, t64525: f64, t15044: f64, t4869: f64, t18910: f64, t3378: f64, t63446: f64, t63449: f64, t63451: f64, t63557: f64, t63560: f64, t63563: f64, t64514: f64, t64517: f64, t64520: f64, t64522: f64, t64524: f64) -> (f64, f64, f64, f64) {
    let t64528 = 0.34631718211362927518e2_f64 * t1164 * t64525 * t4883;
    let t64530 = 0.23392894490538584828e1_f64 * t4869 * t15044;
    let t64533 = 0.35089341735807877242e1_f64 * t1164 * t18910 * t3378;
    let t64534 = t63446 - t63449 + t63451 + t64514 - t64517 - t64520 + t64522 - t64524 - t64528 + t64530 - t64533 + t63557 + t63560 - t63563;
    (t64528, t64530, t64533, t64534)
}
