//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1161/1322 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk1161(t7580: f64, t92247: f64, t7589: f64, t26580: f64, t26623: f64, t2140: f64, t2381: f64, t3110: f64, t1075: f64, t9232: f64, t26597: f64, t26615: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t92256 = t7580 * t92247;
    let t92258 = t7589 * t92247;
    let t92260 = t26580 * t26623;
    let t92263 = t2381 * t3110 * t2140;
    let t92266 = t9232 * t1075 * t2140;
    let t92268 = t26597 * t26615;
    (t92256, t92258, t92260, t92263, t92266, t92268)
}
