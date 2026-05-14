//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 622/1124 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk622<F: Float>(t1058: F, t2207: F, t3606: F, t2608: F, t3332: F, t2147: F, t269: F, t978: F) -> (F, F, F, F) {
    let t3608 = t2207 * t1058 * t3606;
    let t3610 = t3332 * t2608;
    let t3611 = t2147 * t3610;
    let t3613 = t978 * t269;
    (t3608, t3610, t3611, t3613)
}
