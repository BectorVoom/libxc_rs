//! MGGA_C_REVTPSS lxc pol — lxc_pol part 42 (v4rho3tau_5) CSE chunk 1486/1505 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1486<F: Float>(t5876: F, t670: F, t5891: F, t665: F, t1513: F, t4287: F, t5915: F, t5920: F, t648: F, t21881: F, t94: F, t1518: F, t4245: F) -> (F, F, F, F, F, F, F) {
    let t85360 = t5876 * t670;
    let t105872 = t5891 * t665;
    let t105875 = t1513 * t4287;
    let t105880 = t5915 * t665;
    let t108710 = t648 * t5920;
    let t108714 = t94 * t21881;
    let t109150 = t4245 * t1518;
    (t85360, t105872, t105875, t105880, t108710, t108714, t109150)
}
