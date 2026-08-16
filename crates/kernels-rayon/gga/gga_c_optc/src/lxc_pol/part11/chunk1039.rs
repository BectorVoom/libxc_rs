//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1039/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1039(t23682: f64, t23685: f64, t2516: f64, t243: f64, t2519: f64, t24565: f64, t2661: f64, t329: f64, t23548: f64, t7856: f64, t7298: f64, t896: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t24863 = 0.5356037037037037037e1_f64 * t23682;
    let t24864 = 0.16979925925925925926e1_f64 * t23685;
    let t24879 = t2516 * t2516;
    let t24881 = t243 / t24879;
    let t24882 = t2519 * t2519;
    let t24883 = 1.0_f64 / t24882;
    let t24989 = t2661 * t24565;
    let t24995 = t329 * t24565;
    let t25001 = t7856 * t23548;
    let t25085 = t896 * t7298;
    (t24863, t24864, t24881, t24883, t24989, t24995, t25001, t25085)
}
