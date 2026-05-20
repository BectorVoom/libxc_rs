//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2306/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2306<F: Float>(t19979: F, t372: F, t1651: F, t2857: F, t2852: F, t1774: F, t3362: F, t1794: F, t3617: F, t17394: F, t4890: F, t3767: F) -> (F, F, F, F, F, F, F) {
    let t19980 = t372 * t19979;
    let t20094 = t1651 * t2857;
    let t20099 = t1651 * t2852;
    let t20921 = t1774 * t3362;
    let t20944 = t3617 * t1794;
    let t20945 = t372 * t20944;
    let t21013 = t17394 * t4890;
    let t21014 = t3767 * t21013;
    (t19980, t20094, t20099, t20921, t20945, t21013, t21014)
}
