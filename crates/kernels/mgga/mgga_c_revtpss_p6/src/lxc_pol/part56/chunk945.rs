//! MGGA_C_REVTPSS lxc pol — lxc_pol part 56 (v4rho2sigma2_11) CSE chunk 945/1203 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part56_v4rho2sigma2_11_chunk945<F: Float>(t32243: F, t32295: F, t532: F, t1450: F, t2014: F, t7003: F, t8634: F, t32171: F, t508: F, t1310: F, t8454: F, t1459: F, t8611: F) -> (F, F, F, F, F, F, F, F) {
    let t32296 = t32243 + t32295;
    let t32297 = t532 * t32296;
    let t32298 = t32297 * t1450;
    let t32299 = t2014 * t32298;
    let t32320 = F::cast_from(4.0_f64) * t8634 * t7003;
    let t32338 = F::cast_from(2.0_f64) * t32171 * t508;
    let t32340 = F::cast_from(2.0_f64) * t8454 * t1310;
    let t32365 = F::cast_from(6.0_f64) * t1459 * t8611;
    (t32296, t32297, t32298, t32299, t32320, t32338, t32340, t32365)
}
