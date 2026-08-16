//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 658/1222 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk658(t1882: f64, t5176: f64, t5149: f64, t5066: f64, t5172: f64, t8392: f64, t5167: f64, t1160: f64, t2486: f64, t5087: f64, t5083: f64, t5079: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t18427 = t1882 * t5176;
    let t18431 = t1882 * t5149;
    let t18452 = t1882 * t5066;
    let t18455 = t8392 * t5172;
    let t18457 = t8392 * t5167;
    let t18467 = t2486 * t1160;
    let t18538 = t1882 * t5087;
    let t18540 = t1882 * t5083;
    let t18542 = t1882 * t5079;
    (t18427, t18431, t18452, t18455, t18457, t18467, t18538, t18540, t18542)
}
