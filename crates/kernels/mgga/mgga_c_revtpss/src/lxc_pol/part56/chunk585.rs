//! MGGA_C_REVTPSS lxc pol — lxc_pol part 56 (v4rho2sigma2_11) CSE chunk 585/1050 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part56_v4rho2sigma2_11_chunk585<F: Float>(t225: F, t7048: F, t1949: F, t213: F, t1032: F, t251: F, t867: F) -> (F, F, F, F) {
    let t7049 = t7048 * t225;
    let t7053 = t213 * t1949;
    let t7056 = t251 * t1032;
    let t7057 = t7056 * t867;
    (t7049, t7053, t7056, t7057)
}
