//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 835/1276 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk835<F: Float>(t2184: F, t7551: F, t1632: F, t2612: F, t551: F, t1592: F, t552: F, t7195: F, t2832: F, t537: F, t255: F, t571: F) -> (F, F, F, F) {
    let t7553 = F::new(0.46230515946956099004e0) * t2184 * t7551;
    let t7555 = t551 * t1632 * t2612;
    let t7557 = F::new(0.69345773920434148506e0) * t1592 * t7555;
    let t7561 = t551 * t552 * t7195;
    let t7564 = t537 * t2832;
    let t7566 = t571 * t7564 * t255;
    (t7553, t7557, t7561, t7566)
}
