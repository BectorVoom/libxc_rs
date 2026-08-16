//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1202/1323 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk1202(t27491: f64, t6048: f64, t12345: f64, t1555: f64, t28573: f64, t17320: f64, t94833: f64, t48044: f64, t7943: f64, t28644: f64, t4189: f64, t51125: f64, t585: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t97647 = 2.0_f64 * t27491 * t6048;
    let t97650 = 12.0_f64 * t12345 * t28573 * t1555;
    let t97652 = 6.0_f64 * t94833 * t17320;
    let t97654 = 4.0_f64 * t48044 * t7943;
    let t97657 = 4.0_f64 * t4189 * t28644 * t1555;
    let t97661 = t51125 * t585;
    (t97647, t97650, t97652, t97654, t97657, t97661)
}
