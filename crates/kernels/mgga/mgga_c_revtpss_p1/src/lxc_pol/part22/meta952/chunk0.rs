//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3195/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3195<F: Float>(t5219: F, t5477: F, t17288: F, t3754: F, t12722: F, t45785: F, t460: F, t487: F, t45832: F, t5462: F, t1209: F, t21451: F) -> (F, F, F, F, F, F, F) {
    let t59681 = t5219 * t5477;
    let t59686 = t17288 * t3754;
    let t59705 = t5219 * t12722;
    let t59730 = t460 * t45785 * t487;
    let t59737 = t460 * t45832 * t487;
    let t59749 = t5219 * t5462;
    let t59788 = t1209 * t21451;
    (t59681, t59686, t59705, t59730, t59737, t59749, t59788)
}
