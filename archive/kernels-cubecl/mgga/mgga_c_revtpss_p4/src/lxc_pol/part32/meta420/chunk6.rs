//! MGGA_C_REVTPSS lxc pol — lxc_pol part 32 (v4rho3sigma_7) CSE chunk 1472/2056 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1472<F: Float>(t19920: F, t3127: F, t1011: F, t11881: F, t15986: F, t15990: F, t15996: F, t16037: F, t19908: F, t19913: F, t19917: F, t3241: F, t6289: F, t6293: F) -> F {
    let t19921 = t3127 * t19920;
    let t19923 = -t3241 * t6289 / F::cast_from(108.0_f64) + t19908 / F::cast_from(864.0_f64) - t3241 * t6293 / F::cast_from(81.0_f64) + t19913 / F::cast_from(648.0_f64) - t11881 / F::cast_from(1296.0_f64) + t15986 - t15990 + t15996 - t16037 + t1011 * t19917 / F::cast_from(288.0_f64) - F::cast_from(0.19055119163586549765e-3_f64) * t19921;
    t19923
}
