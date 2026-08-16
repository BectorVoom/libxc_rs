//! GGA_C_GAPLOC lxc pol — lxc_pol part 33 (v4rho2sigma2_16) CSE chunk 966/1464 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part33_v4rho2sigma2_16_chunk966(t10802: f64, t5559: f64, t1052: f64, t2728: f64, t1960: f64, t1022: f64, t830: f64, t1: f64, t787: f64, t2631: f64, t2628: f64, t2976: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t10804 = 6.0_f64 * t5559 * t10802;
    let t10805 = t1052 * t2728;
    let t10807 = 2.0_f64 * t1960 * t10805;
    let t10809 = t830 * t1022;
    let t10810 = t10809 * t1;
    let t10811 = t787 * t10810;
    let t10813 = 0.42900587942220512003e1_f64 * t10811 * t2631;
    let t10814 = t2976 * t2628;
    (t10804, t10805, t10807, t10809, t10811, t10813, t10814)
}
