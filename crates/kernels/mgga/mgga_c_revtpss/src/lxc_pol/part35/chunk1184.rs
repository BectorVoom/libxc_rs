//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 1184/1234 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk1184<F: Float>(t1468: F, t6079: F, t1583: F, t5824: F, t113440: F, t27799: F, t100987: F, t29598: F, t113103: F, t25759: F, t113432: F, t1711: F, t5962: F) -> (F, F, F, F, F, F, F) {
    let t113465 = t1468 * t6079;
    let t113484 = t5824 * t1583;
    let t114101 = t27799 * t113440;
    let t114104 = t100987 * t29598;
    let t114107 = t25759 * t113103;
    let t114110 = t25759 * t113432;
    let t114113 = t1711 * t5962;
    (t113465, t113484, t114101, t114104, t114107, t114110, t114113)
}
