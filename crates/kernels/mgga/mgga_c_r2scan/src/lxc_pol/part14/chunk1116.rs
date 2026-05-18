//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 1116/1276 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk1116<F: Float>(t11816: F, t37880: F, t3308: F, t6449: F, t7462: F, t1577: F, t7434: F, t6218: F, t7513: F, t10772: F, t10810: F, t2568: F) -> (F, F, F, F, F) {
    let t39445 = t37880 * t11816;
    let t39448 = t6449 * t3308 * t7462;
    let t39452 = t1577 * t3308 * t7434;
    let t39455 = t6218 * t3308 * t7513;
    let t39458 = t10772 * t10810 * t2568;
    (t39445, t39448, t39452, t39455, t39458)
}
