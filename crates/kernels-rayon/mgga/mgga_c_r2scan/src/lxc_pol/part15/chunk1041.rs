//! MGGA_C_R2SCAN lxc pol — lxc_pol part 15 (v4rho3sigma_5) CSE chunk 1041/1253 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part15_v4rho3sigma_5_chunk1041(t352: f64, t8492: f64, t481: f64, t986: f64, t795: f64, t113: f64, t5086: f64, t1065: f64, t1563: f64, t104: f64, t494: f64, t1543: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t31929 = t352 * t8492;
    let t32094 = t986 * t481;
    let t32212 = t986 * t795;
    let t36967 = t113 * t5086;
    let t36968 = t1065 * t1563;
    let t36969 = t36967 * t36968;
    let t36985 = t104 * t494;
    let t36987 = t1065 * t1543;
    (t31929, t32094, t32212, t36969, t36985, t36987)
}
