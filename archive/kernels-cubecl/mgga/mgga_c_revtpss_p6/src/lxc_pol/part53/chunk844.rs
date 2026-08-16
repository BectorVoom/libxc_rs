//! MGGA_C_REVTPSS lxc pol — lxc_pol part 53 (v4rho2sigma2_8) CSE chunk 844/1244 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part53_v4rho2sigma2_8_chunk844<F: Float>(t25352: F, t689: F, t7049: F, t786: F, t789: F, t2471: F, t7018: F, t25331: F, t7058: F, t25309: F, t7063: F, t7060: F) -> (F, F, F, F, F) {
    let t25353 = t689 * t25352;
    let t25355 = t786 * t7049;
    let t25356 = t25355 * t789;
    let t25362 = F::cast_from(0.13009920719177044025e-1_f64) * t7018 * t2471;
    let t25364 = F::cast_from(0.96373646535613327357e-2_f64) * t7058 * t25331;
    let t25365 = t7063 * t25309;
    let t25366 = t25365 * t7060;
    (t25353, t25356, t25362, t25364, t25366)
}
