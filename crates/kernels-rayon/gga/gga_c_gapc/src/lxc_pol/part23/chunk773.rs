//! GGA_C_GAPC lxc pol — lxc_pol part 23 (v4rho2sigma2_2) CSE chunk 773/1308 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part23_v4rho2sigma2_2_chunk773(t200: f64, t9078: f64, t3000: f64, t2996: f64, t126: f64, t1636: f64, t1875: f64, t4940: f64, t8769: f64, t5190: f64, t116: f64, t5294: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t9079 = t9078 * t200;
    let t9080 = t9079 * t3000;
    let t9081 = t2996 * t9080;
    let t9083 = t126 * t1636;
    let t9084 = t1875 * t9083;
    let t9085 = t9084 * t4940;
    let t9087 = t1875 * t8769;
    let t9088 = t9087 * t5190;
    let t9090 = t116 * t5294;
    (t9079, t9080, t9081, t9083, t9085, t9088, t9090)
}
