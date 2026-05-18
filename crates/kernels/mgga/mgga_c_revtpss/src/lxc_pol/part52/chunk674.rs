//! MGGA_C_REVTPSS lxc pol — lxc_pol part 52 (v4rho2sigma2_7) CSE chunk 674/1292 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part52_v4rho2sigma2_7_chunk674<F: Float>(t1052: F, t1971: F, t351: F, t1058: F, t1973: F, t1061: F) -> (F, F, F, F) {
    let t7125 = t1971 * t1052;
    let t7126 = t351 * t7125;
    let t7130 = F::new(0.28582678745379824648e-3) * t1973 * t1058;
    let t7131 = t1971 * t1061;
    (t7125, t7126, t7130, t7131)
}
