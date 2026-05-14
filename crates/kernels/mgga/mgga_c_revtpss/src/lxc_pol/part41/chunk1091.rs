//! MGGA_C_REVTPSS lxc pol — lxc_pol part 41 (v4rho3tau_4) CSE chunk 1091/1356 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1091<F: Float>(t1209: F, t5412: F, t17288: F, t487: F, t5883: F, t648: F, t1501: F, t670: F) -> (F, F, F, F) {
    let t18097 = t1209 * t5412;
    let t18114 = t17288 * t487;
    let t18220 = t648 * t5883;
    let t18227 = t1501 * t670;
    (t18097, t18114, t18220, t18227)
}
