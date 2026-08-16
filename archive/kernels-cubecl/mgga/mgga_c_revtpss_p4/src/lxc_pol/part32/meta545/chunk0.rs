//! MGGA_C_REVTPSS lxc pol — lxc_pol part 32 (v4rho3sigma_7) CSE chunk 1858/2056 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1858<F: Float>(t2062: F, t2769: F, t786: F, t26519: F, t93157: F, t2453: F, t2458: F, t7399: F, t2070: F, t41154: F, t11064: F, t7427: F) -> (F, F, F, F, F) {
    let t95936 = t786 * t2062 * t2769;
    let t95945 = t93157 * t26519;
    let t95948 = t2453 * t7399 * t2458;
    let t95964 = t2070 * t41154;
    let t95976 = t7427 * t11064;
    (t95936, t95945, t95948, t95964, t95976)
}
