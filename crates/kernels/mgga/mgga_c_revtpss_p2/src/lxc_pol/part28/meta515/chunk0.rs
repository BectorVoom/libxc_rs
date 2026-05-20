//! MGGA_C_REVTPSS lxc pol — lxc_pol part 28 (v4rho3sigma_3) CSE chunk 1926/2277 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1926<F: Float>(t4878: F, t7121: F, t4924: F, t7111: F, t1058: F, t7801: F, t1659: F, t7125: F, t1972: F, t4797: F, t4845: F, t7117: F) -> (F, F, F, F, F, F) {
    let t27450 = t4878 * t7121;
    let t27460 = t7111 * t4924;
    let t27462 = t7801 * t1058;
    let t27464 = t1659 * t7125;
    let t27467 = t4797 * t1972;
    let t27471 = t7117 * t4845;
    (t27450, t27460, t27462, t27464, t27467, t27471)
}
