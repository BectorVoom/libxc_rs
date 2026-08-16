//! GGA_C_GAPLOC lxc pol — lxc_pol part 36 (v4rhosigma3_1) CSE chunk 691/1029 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part36_v4rhosigma3_1_chunk691(t12818: f64, t12843: f64, t209: f64, t10283: f64, t921: f64, t3145: f64, t8045: f64, t2798: f64, t3207: f64, t1016: f64, t9243: f64, t3366: f64, t6556: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t12844 = t12818 + t12843;
    let t12845 = t12844 * t209;
    let t12846 = t10283 * t921;
    let t12847 = 2.0_f64 * t12846;
    let t12849 = 2.0_f64 * t8045 * t3145;
    let t12850 = t2798 * t3207;
    let t12851 = t9243 * t1016;
    let t12853 = 4.0_f64 * t6556 * t3366;
    (t12844, t12845, t12847, t12849, t12850, t12851, t12853)
}
