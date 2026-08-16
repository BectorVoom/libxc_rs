//! MGGA_C_REVTPSS lxc pol — lxc_pol part 32 (v4rho3sigma_7) CSE chunk 1943/2056 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1943<F: Float>(t105944: F, t1955: F, t5978: F, t886: F, t1558: F, t231: F, t4533: F, t6048: F, t836: F, t6071: F, t105945: F, t7063: F) -> (F, F, F, F, F, F) {
    let t106275 = t1955 * t105944;
    let t106290 = t5978 * t886;
    let t106302 = t4533 * t1558 * t231;
    let t106360 = t6048 * t836 * t231;
    let t106365 = t6071 * t836 * t231;
    let t106387 = t7063 * t105945;
    (t106275, t106290, t106302, t106360, t106365, t106387)
}
