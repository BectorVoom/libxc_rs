//! MGGA_C_R2SCAN lxc pol — lxc_pol part 6 (v4rho4_1) CSE chunk 1142/1462 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part6_v4rho4_1_chunk1142<F: Float>(t20818: F, t20820: F, t2157: F, t146: F, t2078: F, t2145: F, t2151: F, t19977: F, t6363: F, t2115: F, t2155: F, t1616: F, t2185: F, t5103: F, t785: F, t122: F, t6159: F, t6161: F) -> (F, F, F, F, F, F, F) {
    let t20822 = t20818 * t2157 * t20820;
    let t20825 = t146 * t2145 * t2078;
    let t20826 = t20825 * t2151;
    let t20828 = t19977 * t6363;
    let t20829 = t2115 * t20828;
    let t20830 = t2155 * t20829;
    let t20834 = t5103 * t785 * t1616 * t2185;
    let t20837 = t6161 * t6159 * t122;
    (t20822, t20825, t20826, t20829, t20830, t20834, t20837)
}
