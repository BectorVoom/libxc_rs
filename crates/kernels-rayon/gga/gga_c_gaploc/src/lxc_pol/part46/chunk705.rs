//! GGA_C_GAPLOC lxc pol — lxc_pol part 46 (v4rhosigma3_11) CSE chunk 705/1029 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part46_v4rhosigma3_11_chunk705(t13008: f64, t1445: f64, t2087: f64, t3009: f64, t3234: f64, t1645: f64, t3255: f64, t3025: f64, t10782: f64, t935: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t13009 = t1445 * t13008;
    let t13010 = t2087 * t13009;
    let t13012 = t3009 * t3234;
    let t13013 = t1445 * t13012;
    let t13015 = 0.69017266717057349418e1_f64 * t2087 * t13013;
    let t13016 = t1645 * t3255;
    let t13018 = 0.10725146985555128001e1_f64 * t3025 * t13016;
    let t13019 = t10782 * t935;
    (t13009, t13010, t13012, t13013, t13015, t13016, t13018, t13019)
}
