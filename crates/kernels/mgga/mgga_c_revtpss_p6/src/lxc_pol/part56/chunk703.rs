//! MGGA_C_REVTPSS lxc pol — lxc_pol part 56 (v4rho2sigma2_11) CSE chunk 703/1203 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part56_v4rho2sigma2_11_chunk703<F: Float>(t1936: F, t4248: F, t1518: F, t93: F, t1312: F, t7741: F, t1847: F, t196: F, t197: F) -> (F, F, F, F, F) {
    let t7888 = F::cast_from(2.0_f64) * t4248 * t1936;
    let t7889 = t93 * t1518;
    let t7891 = F::cast_from(2.0_f64) * t7889 * t1936;
    let t7893 = F::cast_from(2.0_f64) * t1312 * t7741;
    let t7897 = t1847 * t196;
    let t7898 = t7897 * t197;
    (t7888, t7889, t7891, t7893, t7898)
}
