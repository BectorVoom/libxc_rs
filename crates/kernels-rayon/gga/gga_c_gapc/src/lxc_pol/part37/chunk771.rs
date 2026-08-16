//! GGA_C_GAPC lxc pol — lxc_pol part 37 (v4rho2sigma2_16) CSE chunk 771/1445 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part37_v4rho2sigma2_16_chunk771(t1013: f64, t1758: f64, t3079: f64, t561: f64, t1019: f64, t1776: f64, t19: f64, t3071: f64, t1971: f64, t2993: f64, t144: f64, t147: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t8830 = t1013 * t1758;
    let t8832 = t561 * t3079;
    let t8833 = t8832 * t1019;
    let t8835 = t1013 * t1776;
    let t8837 = t3071 * t19;
    let t8838 = t1971 * t8837;
    let t8839 = t2993 * t8838;
    let t8840 = t147 * t144;
    (t8830, t8832, t8833, t8835, t8837, t8838, t8839, t8840)
}
