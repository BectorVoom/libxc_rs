//! GGA_C_FT97 kxc pol — kxc_pol part 2 (v3rho3_1) CSE chunk 747/1007 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_kxc_pol_part2_v3rho3_1_chunk747(t1775: f64, t3151: f64, t3146: f64, t10998: f64, t3134: f64, t11003: f64, t10994: f64, t1787: f64, t3131: f64, t11050: f64, t11046: f64, t1555: f64, t26: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t11732 = 4.0_f64 / 3.0_f64 * t1775 * t3151;
    let t11734 = 2.0_f64 / 9.0_f64 * t1775 * t3146;
    let t11735 = t3134 * t10998;
    let t11738 = t3134 * t11003;
    let t11741 = t1787 * t10994;
    let t11745 = 2.0_f64 / 9.0_f64 * t1775 * t3131;
    let t11746 = t3134 * t11050;
    let t11749 = t1787 * t11046;
    let t11755 = t26 * t1555;
    (t11732, t11734, t11735, t11738, t11741, t11745, t11746, t11749, t11755)
}
