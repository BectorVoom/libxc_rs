//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2589/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2589<F: Float>(t17191: F, t3555: F, t1209: F, t21455: F, t5219: F, t5477: F, t17288: F, t3754: F, t12722: F, t45785: F, t460: F, t487: F) -> (F, F, F, F, F, F) {
    let t59657 = t3555 * t17191;
    let t59674 = t1209 * t21455;
    let t59681 = t5219 * t5477;
    let t59686 = t17288 * t3754;
    let t59705 = t5219 * t12722;
    let t59730 = t460 * t45785 * t487;
    (t59657, t59674, t59681, t59686, t59705, t59730)
}
