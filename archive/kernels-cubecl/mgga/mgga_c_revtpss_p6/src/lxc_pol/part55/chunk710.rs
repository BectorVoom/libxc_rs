//! MGGA_C_REVTPSS lxc pol — lxc_pol part 55 (v4rho2sigma2_10) CSE chunk 710/1306 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part55_v4rho2sigma2_10_chunk710<F: Float>(t7284: F, t7515: F, t7289: F, t1444: F, t2097: F, t7296: F, t1398: F, t543: F, t7301: F, t545: F, t7506: F, t2028: F) -> (F, F, F, F, F, F) {
    let t7517 = F::cast_from(0.72280234901709995518e-2_f64) * t7284 * t7515;
    let t7519 = F::cast_from(0.12851425765524037203e-1_f64) * t7289 * t7515;
    let t7522 = t2097 * t1444;
    let t7523 = t7296 * t7522;
    let t7527 = t2097 * t1398 * t543;
    let t7528 = t7301 * t7527;
    let t7531 = t545 * t7506;
    let t7532 = t2028 * t7531;
    (t7517, t7519, t7523, t7528, t7531, t7532)
}
