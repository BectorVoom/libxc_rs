//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 983/1093 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk983<F: Float>(t2071: F, t5966: F, t1544: F, t1583: F, t1940: F, t198: F, t207: F, t2403: F, t26590: F, t28460: F, t29598: F, t30419: F, t4541: F, t5962: F, t6075: F, t6079: F, t7432: F, t8020: F, t892: F) -> (F,) {
    let t30439 = t2071 * t5966;
    let t30462 = t198 * t207 * t30419 * t892 + 6.0 * t1544 * t2403 * t8020 - 2.0 * t1583 * t1940 * t28460 + 2.0 * t1940 * t26590 * t6079 - t1940 * t6075 * t7432 + 3.0 * t2071 * t2403 * t5962 - 6.0 * t2403 * t29598 * t7432 + 6.0 * t30439 * t4541;
    (t30462,)
}
