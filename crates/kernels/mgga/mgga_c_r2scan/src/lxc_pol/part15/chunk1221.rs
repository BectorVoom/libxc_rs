//! MGGA_C_R2SCAN lxc pol — lxc_pol part 15 (v4rho3sigma_5) CSE chunk 1221/1253 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part15_v4rho3sigma_5_chunk1221<F: Float>(t10966: F, t1103: F, t269: F, t955: F, t10663: F, t40276: F, t11505: F, t494: F, t97: F, t10669: F, t2330: F, t23791: F) -> (F, F, F, F) {
    let t40659 = t10966 * t1103 * t955 * t269;
    let t40662 = t40276 * t10663 / F::new(2.0);
    let t40664 = t97 * t11505 * t494;
    let t40666 = F::new(3.0) / F::new(2.0) * t40664 * t10669;
    let t40667 = t23791 * t2330;
    (t40659, t40662, t40666, t40667)
}
