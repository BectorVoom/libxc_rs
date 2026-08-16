//! GGA_C_ACGGAP lxc pol — lxc_pol part 16 (v4rho3sigma_8) CSE chunk 1018/1223 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part16_v4rho3sigma_8_chunk1018(t1446: f64, t7614: f64, t1988: f64, t8978: f64, t1089: f64, t1579: f64, t2079: f64, t2080: f64, t31276: f64, t8544: f64, t7310: f64, t8505: f64) -> (f64, f64, f64, f64, f64) {
    let t35926 = t7614 * t1446;
    let t35927 = 0.32012600194825403606e-1_f64 * t35926;
    let t35930 = t1988 * t8978;
    let t35931 = 0.42874018118069736972e-3_f64 * t35930;
    let t35934 = t2079 * t1089 * t1579 * t2080;
    let t35935 = 0.21437009059034868486e-3_f64 * t35934;
    let t35936 = t31276 * t8544;
    let t35938 = t7310 * t8505;
    (t35927, t35931, t35935, t35936, t35938)
}
