//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 297/1086 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk297<F: Float>(t1634: F, t45: F, t591: F, t596: F) -> (F, F, F, F) {
    let t1668 = F::cast_from(0.92708333333333333333e-2_f64) * t1634;
    let t1674 = t45 * t591;
    let t1675 = t596 * t596;
    let t1676 = F::cast_from(1.0_f64) / t1675;
    (t1668, t1674, t1675, t1676)
}
