//! MGGA_C_R2SCAN lxc pol — lxc_pol part 15 (v4rho3sigma_5) CSE chunk 1007/1115 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part15_v4rho3sigma_5_chunk1007<F: Float>(t1577: F, t3308: F, t7434: F, t6218: F, t7513: F, t10772: F, t10810: F, t2568: F, t11808: F, t37685: F, t11811: F, t37641: F, t10768: F, t8129: F, t2196: F, t24790: F) -> (F, F, F, F, F, F, F) {
    let t39452 = t1577 * t3308 * t7434;
    let t39455 = t6218 * t3308 * t7513;
    let t39458 = t10772 * t10810 * t2568;
    let t39459 = 0.69345773920434148506e0 * t39458;
    let t39460 = t37685 * t11808;
    let t39462 = t37641 * t11811;
    let t39464 = t10768 * t8129;
    let t39467 = t2196 * t3308 * t24790;
    (t39452, t39455, t39459, t39460, t39462, t39464, t39467)
}
