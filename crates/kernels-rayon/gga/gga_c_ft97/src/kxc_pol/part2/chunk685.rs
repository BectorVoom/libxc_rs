//! GGA_C_FT97 kxc pol — kxc_pol part 2 (v3rho3_1) CSE chunk 685/1007 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_kxc_pol_part2_v3rho3_1_chunk685(t1882: f64, t2751: f64, t869: f64, t309: f64, t2770: f64, t871: f64, t2869: f64, t8232: f64, t837: f64, t877: f64, t2834: f64, t681: f64, t89: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t10693 = t1882 * t2751;
    let t10695 = t869 * t869;
    let t10696 = 1.0_f64 / t10695;
    let t10697 = t309 * t10696;
    let t10703 = t2770 * t871;
    let t10730 = t1882 * t2869;
    let t10732 = t8232 * t837;
    let t10735 = t8232 * t877;
    let t10745 = t89 * t681 * t2834;
    (t10693, t10697, t10703, t10730, t10732, t10735, t10745)
}
