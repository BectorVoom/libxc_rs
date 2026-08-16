//! MGGA_C_KCIS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 1074/1420 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part4_v3rho3_1_chunk1074(t13589: f64, t4882: f64, t1737: f64, t2471: f64, t1742: f64, t2475: f64, t1734: f64, t2466: f64, t1068: f64, t1646: f64, t10108: f64, t1056: f64, t13475: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t13590 = t13589 * t4882;
    let t13592 = t2471 * t1737;
    let t13594 = t2475 * t1742;
    let t13596 = t2466 * t1734;
    let t13598 = t1068 * t1646;
    let t13600 = t10108 * t1646;
    let t13602 = t1056 * t13475;
    (t13590, t13592, t13594, t13596, t13598, t13600, t13602)
}
