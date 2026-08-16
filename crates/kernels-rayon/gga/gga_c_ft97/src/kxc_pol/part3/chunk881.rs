//! GGA_C_FT97 kxc pol — kxc_pol part 3 (v3rho3_2) CSE chunk 881/1032 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_kxc_pol_part3_v3rho3_2_chunk881(t147: f64, t16615: f64, t17679: f64, t1526: f64, t4906: f64, t9483: f64, t10915: f64, t240: f64, t3691: f64, t2917: f64, t3700: f64, t18: f64, t2321: f64) -> (f64, f64, f64, f64, f64) {
    let t148 = 10000000.0_f64 <= t147;
    let t17681 = piecewise3(t148, 0.0_f64, t16615 + t17679);
    let t17685 = t1526 * t9483 * t4906;
    let t17687 = t10915 * t240;
    let t17688 = t17687 * t3691;
    let t17694 = t2917 * t240;
    let t17695 = t17694 * t3700;
    let t17698 = t2321 * t18;
    (t17681, t17685, t17688, t17695, t17698)
}
