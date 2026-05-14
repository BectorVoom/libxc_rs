//! MGGA_C_REVTPSS lxc pol — lxc_pol part 56 (v4rho2sigma2_11) CSE chunk 886/1050 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part56_v4rho2sigma2_11_chunk886<F: Float>(t1892: F, t8477: F, t1903: F, t8578: F, t32250: F, t1882: F, t543: F, t32255: F, t2022: F, t7910: F, t8707: F, t8590: F, t552: F, t125: F, t246: F, t551: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t33943 = t8477 * t1892;
    let t33946 = t8578 * t1903;
    let t33947 = t32250 * t33946;
    let t33951 = t8578 * t1882 * t543;
    let t33952 = t32255 * t33951;
    let t33955 = t2022 * t7910;
    let t33956 = t8707 * t33955;
    let t33959 = t33943 * t8590;
    let t33960 = t33959 * t552;
    let t33962 = t125 * t1903;
    let t33963 = t246 * t33962;
    let t33964 = t551 * t33963;
    (t33943, t33946, t33947, t33951, t33952, t33955, t33956, t33959, t33960, t33962, t33963, t33964)
}
