//! MGGA_C_REVTPSS lxc pol — lxc_pol part 51 (v4rho2sigma2_6) CSE chunk 1044/1200 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part51_v4rho2sigma2_6_chunk1044(t120248: f64, t3148: f64, t31998: f64, t120237: f64, t31903: f64, t1061: f64, t31891: f64, t31892: f64, t3143: f64, t3268: f64, t1039: f64, t31997: f64) -> (f64, f64, f64, f64) {
    let t120256 = t120248 * t31998 * t3148;
    let t120259 = t31903 * t120237;
    let t120263 = t31891 * t31892 * t1061;
    let t120273 = t3268 * t3143;
    let t120275 = t31997 * t120273 * t1039;
    (t120256, t120259, t120263, t120275)
}
