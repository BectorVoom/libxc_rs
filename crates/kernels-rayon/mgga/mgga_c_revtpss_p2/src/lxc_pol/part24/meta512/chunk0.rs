//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1528/1850 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1528(t11922: f64, t11927: f64, t23838: f64, t23998: f64, t3115: f64, t23916: f64, t3091: f64, t43131: f64, t15618: f64, t19785: f64, t23820: f64, t3153: f64) -> (f64, f64, f64, f64, f64) {
    let t78802 = t11927 * t11922 * t23838;
    let t78805 = t3115 * t11922 * t23998;
    let t78855 = t3091 * t43131 * t23916;
    let t78863 = t15618 * t19785;
    let t78873 = t23820 * t3153;
    (t78802, t78805, t78855, t78863, t78873)
}
