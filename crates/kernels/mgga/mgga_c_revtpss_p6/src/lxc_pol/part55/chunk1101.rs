//! MGGA_C_REVTPSS lxc pol — lxc_pol part 55 (v4rho2sigma2_10) CSE chunk 1101/1306 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part55_v4rho2sigma2_10_chunk1101<F: Float>(t2035: F, t34270: F, t7984: F, t8634: F, t7359: F, t7742: F, t1907: F, t2033: F, t28286: F, t28196: F, t1868: F, t26405: F) -> (F, F, F, F, F, F, F, F) {
    let t34271 = t34270 * t2035;
    let t34285 = F::new(2.0) * t8634 * t7984;
    let t34294 = F::new(2.0) * t7359 * t7742;
    let t34297 = t2033 * t1907;
    let t34298 = t28286 * t34297;
    let t34300 = F::new(2.0) * t28196 * t34298;
    let t34301 = t2033 * t1868;
    let t34302 = t26405 * t34301;
    (t34271, t34285, t34294, t34297, t34298, t34300, t34301, t34302)
}
