//! MGGA_C_REVTPSS lxc pol — lxc_pol part 39 (v4rho3tau_2) CSE chunk 1258/1507 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1258<F: Float>(t15098: F, t2924: F, t1596: F, t2873: F, t2876: F, t1614: F, t2942: F, t11354: F, t1600: F, t2881: F, t11358: F, t2880: F, t4606: F) -> (F, F, F, F, F, F) {
    let t15100 = F::cast_from(6.0_f64) * t2924 * t15098;
    let t15101 = t1596 * t2873;
    let t15103 = F::cast_from(2.0_f64) * t15101 * t2876;
    let t15104 = t1614 * t2942;
    let t15107 = t11354 * t1600;
    let t15108 = t15107 * t2881;
    let t15110 = t11358 * t1600;
    let t15111 = t15110 * t2881;
    let t15113 = t2880 * t4606;
    (t15100, t15103, t15104, t15108, t15111, t15113)
}
