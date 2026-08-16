//! GGA_C_GAPLOC lxc pol — lxc_pol part 38 (v4rhosigma3_3) CSE chunk 595/1003 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part38_v4rhosigma3_3_chunk595(t10900: f64, t10876: f64, t10878: f64, t10881: f64, t10885: f64, t10888: f64, t10891: f64, t10893: f64, t10899: f64, t2028: f64, t9836: f64, t9838: f64, t9846: f64, t9849: f64, t9853: f64, t9892: f64) -> f64 {
    let t10901 = 0.14896037479937677779e-1_f64 * t10900;
    let t10902 = -t9836 + t9838 - t9846 - t9849 + t9853 - t10876 + t10878 + t10881 - t10885 + t10888 - t10891 - 0.39722766613167140743e-1_f64 * t10893 * t2028 + t10899 - t9892 + t10901;
    t10902
}
