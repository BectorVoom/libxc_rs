//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 2020/2369 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2020(t24525: f64, t9239: f64, t39063: f64, t7245: f64, t39054: f64, t50: f64, t9300: f64, t11588: f64, t2127: f64, t221: f64) -> (f64, f64, f64, f64, f64) {
    let t85480 = t9239 * t24525;
    let t85501 = t39063 * t7245;
    let t85536 = t39054 * t7245;
    let t85539 = t50 * t9300;
    let t85639 = t2127 * t221 * t11588;
    (t85480, t85501, t85536, t85539, t85639)
}
