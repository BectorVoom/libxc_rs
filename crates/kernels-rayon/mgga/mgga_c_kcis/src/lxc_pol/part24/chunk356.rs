//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 356/1322 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk356(t1282: f64, t1779: f64, t1821: f64, t1864: f64, t187: f64, t1872: f64, t437: f64, t69: f64, t706: f64, t74: f64) -> (f64, f64) {
    let t1876 = t1779 - t1821 + t187 * (-t1282 * t1872 + t1864 * t437 - t1779 + t1821);
    let t2140 = t69 * t74 * t706;
    (t1876, t2140)
}
