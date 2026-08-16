//! MGGA_C_REVTPSS lxc pol — lxc_pol part 32 (v4rho3sigma_7) CSE chunk 1931/2056 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1931<F: Float>(t103421: F, t7058: F, t11064: F, t8019: F, t28993: F, t571: F, t2118: F, t5789: F, t1464: F, t8113: F, t1913: F, t7560: F) -> (F, F, F, F, F, F) {
    let t103547 = t7058 * t103421;
    let t103586 = t8019 * t11064;
    let t104062 = F::cast_from(2.0_f64) * t571 * t28993;
    let t104071 = F::cast_from(2.0_f64) * t5789 * t2118;
    let t104073 = F::cast_from(2.0_f64) * t8113 * t1464;
    let t104077 = F::cast_from(2.0_f64) * t1913 * t7560;
    (t103547, t103586, t104062, t104071, t104073, t104077)
}
