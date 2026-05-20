//! MGGA_C_REVTPSS lxc pol — lxc_pol part 30 (v4rho3sigma_5) CSE chunk 2082/2270 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2082<F: Float>(t1955: F, t25949: F, t1883: F, t4131: F, t1904: F, t25912: F, t689: F, t1903: F, t3923: F, t4003: F, t1385: F, t7910: F) -> (F, F, F, F, F, F) {
    let t97855 = t1955 * t25949;
    let t97858 = t1883 * t4131;
    let t97869 = F::cast_from(0.10975748638225852664e-1_f64) * t689 * t25912 * t1904;
    let t97870 = t1903 * t3923;
    let t97871 = t97870 * t4003;
    let t97875 = t1385 * t7910;
    (t97855, t97858, t97869, t97870, t97871, t97875)
}
