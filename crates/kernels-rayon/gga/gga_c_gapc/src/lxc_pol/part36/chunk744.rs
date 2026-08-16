//! GGA_C_GAPC lxc pol — lxc_pol part 36 (v4rho2sigma2_15) CSE chunk 744/1328 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part36_v4rho2sigma2_15_chunk744(t2998: f64, t8847: f64, t2996: f64, t1030: f64, t8838: f64, t3006: f64, t5298: f64, t8841: f64, t1896: f64, t3034: f64, t1026: f64, t1804: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t8848 = t2998 * t8847;
    let t8849 = t2996 * t8848;
    let t8851 = t1030 * t8838;
    let t8852 = t5298 * t3006;
    let t8853 = t8841 * t8852;
    let t8854 = t8851 * t8853;
    let t8856 = t3034 * t1896;
    let t8858 = t1804 * t1026;
    (t8848, t8849, t8853, t8854, t8856, t8858)
}
