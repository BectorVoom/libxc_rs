//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 337/1063 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk337<F: Float>(t1670: F, t600: F, t45: F, t591: F, t596: F) -> (F, F, F, F) {
    let t1671 = t1670 * t600;
    let t1674 = t45 * t591;
    let t1675 = t596 * t596;
    let t1676 = F::cast_from(1.0_f64) / t1675;
    (t1671, t1674, t1675, t1676)
}
