//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 821/1322 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk821(t1134: f64, t18463: f64, t1092: f64, t6487: f64, t9532: f64, t13192: f64, t4807: f64, t2825: f64, t6629: f64, t1020: f64, t2811: f64, t6544: f64) -> (f64, f64, f64, f64, f64) {
    let t18464 = t18463 * t1134;
    let t18465 = t1092 * t18464;
    let t18467 = t9532 * t6487;
    let t18468 = t1092 * t18467;
    let t18471 = t13192 * t4807;
    let t18473 = t2825 * t6629;
    let t18474 = t1020 * t18473;
    let t18476 = t6544 * t2811;
    (t18465, t18468, t18471, t18474, t18476)
}
