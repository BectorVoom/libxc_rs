//! MGGA_C_REVTPSS lxc pol — lxc_pol part 53 (v4rho2sigma2_8) CSE chunk 1056/1089 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part53_v4rho2sigma2_8_chunk1056<F: Float>(t2040: F, t28265: F, t28280: F, t5795: F, t8614: F, t7334: F, t7944: F, t7324: F, t7953: F, t7950: F, t1459: F, t34007: F, t1916: F, t32366: F, t121661: F, t125336: F) -> (F, F, F, F, F, F, F, F, F) {
    let t127490 = t2040 * t28265;
    let t127492 = t2040 * t28280;
    let t127495 = 3.0 * t5795 * t8614;
    let t127496 = t7944 * t7334;
    let t127498 = t7324 * t7953;
    let t127500 = t7324 * t7950;
    let t127503 = 12.0 * t1459 * t34007;
    let t127507 = 6.0 * t1916 * t32366;
    let t128368 = t121661 * t125336;
    (t127490, t127492, t127495, t127496, t127498, t127500, t127503, t127507, t128368)
}
