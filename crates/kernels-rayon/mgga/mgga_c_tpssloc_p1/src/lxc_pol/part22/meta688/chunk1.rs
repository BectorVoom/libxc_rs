//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2266/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2266(t18469: f64, t3447: f64, t44525: f64, t18206: f64, t52133: f64, t4899: f64, t6138: f64, t6144: f64, t15376: f64, t15420: f64, t15419: f64, t18211: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t64627 = t3447 * t44525 * t18469;
    let t64632 = t3447 * t52133 * t18206;
    let t64644 = t4899 * t6138;
    let t64648 = t4899 * t6144;
    let t64667 = t15376 * t15420;
    let t64686 = t3447 * t15419 * t18211;
    (t64627, t64632, t64644, t64648, t64667, t64686)
}
