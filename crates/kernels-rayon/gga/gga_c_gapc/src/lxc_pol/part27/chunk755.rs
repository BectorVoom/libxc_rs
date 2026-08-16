//! GGA_C_GAPC lxc pol — lxc_pol part 27 (v4rho2sigma2_6) CSE chunk 755/1310 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part27_v4rho2sigma2_6_chunk755(t200: f64, t8840: f64, t2999: f64, t5319: f64, t8839: f64, t1338: f64, t134: f64, t647: f64, t2998: f64, t2996: f64, t1030: f64, t8838: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t8841 = t8840 * t200;
    let t8842 = t5319 * t2999;
    let t8843 = t8841 * t8842;
    let t8844 = t8839 * t8843;
    let t8846 = t134 * t1338;
    let t8847 = t647 * t8846;
    let t8848 = t2998 * t8847;
    let t8849 = t2996 * t8848;
    let t8851 = t1030 * t8838;
    (t8841, t8843, t8844, t8848, t8849, t8851)
}
