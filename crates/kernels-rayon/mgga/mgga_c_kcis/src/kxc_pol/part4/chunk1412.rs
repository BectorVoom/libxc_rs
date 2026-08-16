//! MGGA_C_KCIS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 1412/1420 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_kxc_pol_part4_v3rho3_1_chunk1412(t17307: f64, t17310: f64, t17313: f64, t17314: f64, t17315: f64, t17317: f64, t17319: f64, t17322: f64, t17325: f64, t17328: f64, t17709: f64, t18354: f64, t18364: f64, t187: f64) -> f64 {
    let t18367 = t17307 - t17310 + t17313 - t17314 - t17315 + t17317 - t17319 - t17322 + t17325 + t17328 - t17709 + t187 * (t18354 + t18364);
    t18367
}
