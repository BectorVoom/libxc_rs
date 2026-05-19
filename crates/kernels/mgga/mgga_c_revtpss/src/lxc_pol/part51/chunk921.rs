//! MGGA_C_REVTPSS lxc pol — lxc_pol part 51 (v4rho2sigma2_6) CSE chunk 921/1200 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part51_v4rho2sigma2_6_chunk921<F: Float>(t11922: F, t8523: F, t8522: F, t1078: F, t1096: F, t247: F, t3116: F, t7165: F, t8513: F) -> (F, F, F, F, F) {
    let t31883 = t8523 * t11922;
    let t31885 = F::cast_from(0.12395776403017003607e-3_f64) * t8522 * t31883;
    let t31886 = t1078 * t1096;
    let t31888 = t247 * t3116 * t31886;
    let t31891 = t8513 * t7165;
    (t31883, t31885, t31886, t31888, t31891)
}
