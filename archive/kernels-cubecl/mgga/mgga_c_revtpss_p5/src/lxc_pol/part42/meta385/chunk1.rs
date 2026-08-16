//! MGGA_C_REVTPSS lxc pol — lxc_pol part 42 (v4rho3tau_5) CSE chunk 1271/1505 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1271<F: Float>(t19450: F, t19491: F, t1043: F, t6258: F, t1089: F, t3153: F, t6305: F) -> (F, F, F, F) {
    let t19492 = t19450 * t19491;
    let t19497 = t6258 * t1043;
    let t19498 = t19497 * t1089;
    let t19501 = t6305 * t3153;
    (t19492, t19497, t19498, t19501)
}
