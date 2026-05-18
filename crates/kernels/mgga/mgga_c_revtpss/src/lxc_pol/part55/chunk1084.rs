//! MGGA_C_REVTPSS lxc pol — lxc_pol part 55 (v4rho2sigma2_10) CSE chunk 1084/1306 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part55_v4rho2sigma2_10_chunk1084<F: Float>(t32206: F, t33930: F, t1892: F, t8477: F, t8590: F, t552: F, t125: F, t1903: F, t246: F, t551: F, t32276: F, t1885: F, t32284: F) -> (F, F, F, F, F, F, F, F, F) {
    let t33931 = t32206 * t33930;
    let t33943 = t8477 * t1892;
    let t33959 = t33943 * t8590;
    let t33960 = t33959 * t552;
    let t33962 = t125 * t1903;
    let t33963 = t246 * t33962;
    let t33964 = t551 * t33963;
    let t33965 = t32276 * t33964;
    let t33967 = t32284 * t1885;
    (t33931, t33943, t33959, t33960, t33962, t33963, t33964, t33965, t33967)
}
