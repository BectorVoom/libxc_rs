//! MGGA_C_REVTPSS lxc pol — lxc_pol part 32 (v4rho3sigma_7) CSE chunk 1717/2056 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1717<F: Float>(t27261: F, t4368: F, t1955: F, t4469: F, t1579: F, t231: F, t836: F, t1559: F, t886: F, t7057: F) -> (F, F, F, F, F) {
    let t27262 = t27261 * t4368;
    let t27275 = t1955 * t4469;
    let t27312 = t1579 * t836 * t231;
    let t27349 = t1559 * t886;
    let t27353 = t1955 * t7057;
    (t27262, t27275, t27312, t27349, t27353)
}
