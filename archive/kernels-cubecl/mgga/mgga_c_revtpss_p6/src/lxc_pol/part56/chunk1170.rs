//! MGGA_C_REVTPSS lxc pol — lxc_pol part 56 (v4rho2sigma2_11) CSE chunk 1170/1203 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part56_v4rho2sigma2_11_chunk1170<F: Float>(t8142: F, t84: F, t33358: F, t60224: F, t13272: F, t124503: F, t124508: F, t125228: F, t125244: F, t125248: F, t125328: F, t29362: F, t32138: F, t32156: F, t32795: F, t32798: F, t32806: F, t33359: F, t33367: F, t33370: F, t33613: F, t33624: F, t33625: F, t34402: F, t34866: F, t34867: F, t640: F, t644: F, t7574: F, t8441: F, t8621: F, t8737: F) -> F {
    let t131281 = t84 * t8142;
    let t131292 = t60224 * t33358;
    let t131297 = t13272 * t33358;
    let t131318 = -F::cast_from(5.0_f64) / F::cast_from(12.0_f64) * t32798 * t8621 * t34866 * t644 + F::cast_from(5.0_f64) / F::cast_from(36.0_f64) * t8737 * t8621 * t131281 * t640 - F::cast_from(5.0_f64) / F::cast_from(12.0_f64) * t33359 * t125328 + F::cast_from(5.0_f64) / F::cast_from(36.0_f64) * t8737 * t8621 * t33624 * t7574 - F::cast_from(5.0_f64) / F::cast_from(24.0_f64) * t131292 * t32138 + F::cast_from(5.0_f64) / F::cast_from(72.0_f64) * t34402 * t33367 + F::cast_from(5.0_f64) / F::cast_from(72.0_f64) * t131297 * t32156 - F::cast_from(5.0_f64) / F::cast_from(24.0_f64) * t124503 * t33613 - F::cast_from(5.0_f64) / F::cast_from(24.0_f64) * t33359 * t125228 + F::cast_from(5.0_f64) / F::cast_from(72.0_f64) * t32795 * t34867 + F::cast_from(5.0_f64) / F::cast_from(72.0_f64) * t32806 * t34867 + F::cast_from(5.0_f64) / F::cast_from(72.0_f64) * t8737 * t8621 * t8441 * t29362 + F::cast_from(5.0_f64) / F::cast_from(72.0_f64) * t124508 * t33625 + F::cast_from(5.0_f64) / F::cast_from(72.0_f64) * t33370 * t125244 + F::cast_from(5.0_f64) / F::cast_from(72.0_f64) * t33370 * t125248;
    t131318
}
