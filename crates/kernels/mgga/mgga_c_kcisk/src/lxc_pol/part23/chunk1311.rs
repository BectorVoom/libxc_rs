//! MGGA_C_KCISK lxc pol — lxc_pol part 23 (v4rho3sigma_3) CSE chunk 1311/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part23_v4rho3sigma_3_chunk1311<F: Float>(t111245: F, t15220: F, t3052: F, t31895: F, t111233: F, t31902: F, t127: F, t397: F, t43200: F, t932: F, t31894: F, t31898: F, t856: F, t911: F, t2932: F, t1394: F, t20: F) -> (F, F, F, F, F, F, F, F) {
    let t111252 = t111245 * t15220 * t3052;
    let t111253 = t31895 * t111252;
    let t111255 = t31902 * t111233;
    let t111259 = t397 * t127 * t43200 * t932;
    let t111260 = t31895 * t111259;
    let t111264 = t856 * t911 * t31894 * t31898;
    let t111268 = t2932 * t911 * t31894 * t31898;
    let t111270 = t1394 * t20;
    (t111252, t111253, t111255, t111259, t111260, t111264, t111268, t111270)
}
