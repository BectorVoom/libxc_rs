//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 24 (v4rho3sigma_0) CSE chunk 1310/1438 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part24_v4rho3sigma_0_chunk1310(t10098: f64, t1888: f64, t6646: f64, t229: f64, t268: f64, t6559: f64, t22988: f64, t23110: f64, t22893: f64, t23154: f64, t23164: f64, t234: f64, t2710: f64) -> (f64, f64, f64, f64, f64) {
    let t81648 = t1888 * t6646 * t10098;
    let t81651 = t6559 * t229 * t268;
    let t81653 = t81651 * t23110 * t22988;
    let t81656 = t23164 * t22893 * t23154;
    let t81658 = t234 * t2710;
    (t81648, t81651, t81653, t81656, t81658)
}
