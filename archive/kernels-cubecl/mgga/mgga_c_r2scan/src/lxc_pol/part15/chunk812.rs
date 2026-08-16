//! MGGA_C_R2SCAN lxc pol — lxc_pol part 15 (v4rho3sigma_5) CSE chunk 812/1253 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part15_v4rho3sigma_5_chunk812<F: Float>(t108: F, t7151: F, t7152: F, t7154: F, t7162: F, t486: F, t95: F, t5052: F, t910: F, t1543: F, t1541: F, t2526: F) -> (F, F, F, F) {
    let t7165 = (t7151 + t7152 + t7154 + t7162) * t108;
    let t7175 = t486 * t95;
    let t7180 = t5052 * t910;
    let t7181 = t7180 * t1543;
    let t7184 = t1541 * t2526;
    (t7165, t7175, t7181, t7184)
}
