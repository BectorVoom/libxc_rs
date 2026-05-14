//! MGGA_C_REVTPSS lxc pol — lxc_pol part 39 (v4rho3tau_2) CSE chunk 898/1348 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk898<F: Float>(t2782: F, t5737: F, t1883: F, t72: F, t686: F, t4101: F, t225: F, t3999: F, t213: F, t4086: F, t1892: F, t545: F, t869: F, t689: F, t1432: F, t1385: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t5738 = t2782 * t5737;
    let t5740 = t1883 * t72;
    let t5741 = t5740 * t686;
    let t5742 = t4101 * t5741;
    let t5744 = t225 * t3999;
    let t5745 = t213 * t5744;
    let t5755 = t213 * t4086;
    let t5759 = t545 * t1892;
    let t5760 = t869 * t5759;
    let t5761 = t689 * t5760;
    let t5763 = t1892 * t72;
    let t5765 = t1432 * t5763 * t686;
    let t5767 = t1385 * t1892;
    (t5738, t5740, t5741, t5742, t5744, t5745, t5755, t5759, t5760, t5761, t5763, t5765, t5767)
}
