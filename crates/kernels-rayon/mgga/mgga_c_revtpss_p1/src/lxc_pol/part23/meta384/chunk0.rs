//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 1729/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1729(t16712: f64, t12256: f64, t1469: f64, t3362: f64, t4186: f64, t3367: f64, t3153: f64, t5284: f64, t300: f64, t5155: f64, t16710: f64, t16708: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t16713 = 0.9877777777777777778e-2_f64 * t16712;
    let t16714 = t12256 * t1469;
    let t16724 = t3362 * t4186;
    let t16737 = t3367 * t4186;
    let t16756 = t5284 * t3153;
    let t16784 = t300 * t5155;
    let t16797 = 0.23744444444444444444e-1_f64 * t16710;
    let t16798 = 0.11872222222222222222e-1_f64 * t16712;
    let t16820 = 0.41203703703703703704e-2_f64 * t16708;
    (t16713, t16714, t16724, t16737, t16756, t16784, t16797, t16798, t16820)
}
