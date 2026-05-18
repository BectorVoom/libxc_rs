//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 882/1276 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk882<F: Float>(t2294: F, t2599: F, t2598: F, t1543: F, t910: F, t551: F, t552: F, t1632: F, t2625: F, t2196: F, t2526: F, t560: F) -> (F, F, F, F, F) {
    let t8044 = t2294 * t2599;
    let t8046 = F::new(0.46230515946956099004e0) * t2598 * t8044;
    let t8048 = t910 * t1543;
    let t8050 = t551 * t552 * t8048;
    let t8054 = t551 * t1632 * t2625;
    let t8056 = F::new(0.27738309568173659402e1) * t2196 * t8054;
    let t8057 = t2526 * t560;
    (t8046, t8048, t8050, t8056, t8057)
}
