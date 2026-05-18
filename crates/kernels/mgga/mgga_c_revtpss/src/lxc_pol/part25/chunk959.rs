//! MGGA_C_REVTPSS lxc pol — lxc_pol part 25 (v4rho3sigma_0) CSE chunk 959/1360 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part25_v4rho3sigma_0_chunk959<F: Float>(t1042: F, t11280: F, t2853: F, t3181: F, t999: F, t2866: F, t914: F, t936: F, t2869: F, t2919: F, t2923: F, t910: F) -> (F, F, F, F, F) {
    let t11281 = t1042 * t11280;
    let t11285 = t3181 * t999 * t2853;
    let t11286 = t1042 * t11285;
    let t11289 = t2866 * t914;
    let t11291 = F::new(3.0) * t11289 * t936;
    let t11293 = F::new(3.0) * t2869 * t2919;
    let t11294 = t910 * t2923;
    (t11281, t11286, t11291, t11293, t11294)
}
