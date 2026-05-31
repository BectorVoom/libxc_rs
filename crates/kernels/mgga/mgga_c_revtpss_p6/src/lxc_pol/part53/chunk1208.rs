//! MGGA_C_REVTPSS lxc pol — lxc_pol part 53 (v4rho2sigma2_8) CSE chunk 1208/1244 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part53_v4rho2sigma2_8_chunk1208<F: Float>(t7324: F, t7950: F, t1459: F, t34007: F, t1916: F, t32366: F, t121661: F, t125336: F, t125260: F, t121656: F, t125268: F, t125279: F) -> (F, F, F, F, F, F, F) {
    let t127500 = t7324 * t7950;
    let t127503 = F::cast_from(12.0_f64) * t1459 * t34007;
    let t127507 = F::cast_from(6.0_f64) * t1916 * t32366;
    let t128368 = t121661 * t125336;
    let t128371 = t121661 * t125260;
    let t128374 = t121656 * t125268;
    let t128377 = t121656 * t125279;
    (t127500, t127503, t127507, t128368, t128371, t128374, t128377)
}
