//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3045/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3045(t14575: f64, t2435: f64, t10943: f64, t14598: f64, t686: f64, t72: f64, t10541: f64, t14495: f64, t2782: f64, t10518: f64, t14568: f64, t1568: f64, t4503: f64) -> (f64, f64, f64, f64, f64) {
    let t51537 = t2435 * t14575;
    let t51541 = t14598 * t10943 * t72 * t686;
    let t51544 = t2782 * t10541 * t14495;
    let t51546 = t14568 * t10518;
    let t51548 = t4503 * t1568;
    (t51537, t51541, t51544, t51546, t51548)
}
