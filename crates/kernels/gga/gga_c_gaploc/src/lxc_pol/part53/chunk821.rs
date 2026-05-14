//! GGA_C_GAPLOC lxc pol — lxc_pol part 53 (v4rhosigma3_18) CSE chunk 821/923 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part53_v4rhosigma3_18_chunk821<F: Float>(t2268: F, t426: F, t46849: F, t535: F, t13740: F, t484: F, t11977: F, t6763: F, t1063: F, t6750: F, t13756: F, t380: F, t12035: F, t6556: F, t39340: F, t921: F) -> (F, F, F, F, F, F, F) {
    let t47040 = t2268 * t535 * t46849 * t426;
    let t47042 = t484 * t13740;
    let t47047 = t2268 * t11977 * t6763;
    let t47050 = t1063 * t11977 * t6750;
    let t47054 = 0.37940008847568199465e-1 * t380 * t13756;
    let t47064 = t6556 * t12035;
    let t47071 = t39340 * t921;
    (t47040, t47042, t47047, t47050, t47054, t47064, t47071)
}
