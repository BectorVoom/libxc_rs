//! MGGA_C_REVTPSS lxc pol — lxc_pol part 56 (v4rho2sigma2_11) CSE chunk 1170/1203 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part56_v4rho2sigma2_11_chunk1170(t8142: f64, t84: f64, t33358: f64, t60224: f64, t13272: f64, t124503: f64, t124508: f64, t125228: f64, t125244: f64, t125248: f64, t125328: f64, t29362: f64, t32138: f64, t32156: f64, t32795: f64, t32798: f64, t32806: f64, t33359: f64, t33367: f64, t33370: f64, t33613: f64, t33624: f64, t33625: f64, t34402: f64, t34866: f64, t34867: f64, t640: f64, t644: f64, t7574: f64, t8441: f64, t8621: f64, t8737: f64) -> f64 {
    let t131281 = t84 * t8142;
    let t131292 = t60224 * t33358;
    let t131297 = t13272 * t33358;
    let t131318 = -5.0_f64 / 12.0_f64 * t32798 * t8621 * t34866 * t644 + 5.0_f64 / 36.0_f64 * t8737 * t8621 * t131281 * t640 - 5.0_f64 / 12.0_f64 * t33359 * t125328 + 5.0_f64 / 36.0_f64 * t8737 * t8621 * t33624 * t7574 - 5.0_f64 / 24.0_f64 * t131292 * t32138 + 5.0_f64 / 72.0_f64 * t34402 * t33367 + 5.0_f64 / 72.0_f64 * t131297 * t32156 - 5.0_f64 / 24.0_f64 * t124503 * t33613 - 5.0_f64 / 24.0_f64 * t33359 * t125228 + 5.0_f64 / 72.0_f64 * t32795 * t34867 + 5.0_f64 / 72.0_f64 * t32806 * t34867 + 5.0_f64 / 72.0_f64 * t8737 * t8621 * t8441 * t29362 + 5.0_f64 / 72.0_f64 * t124508 * t33625 + 5.0_f64 / 72.0_f64 * t33370 * t125244 + 5.0_f64 / 72.0_f64 * t33370 * t125248;
    t131318
}
