//! MGGA_C_REVTPSS lxc pol — lxc_pol part 34 (v4rho3sigma_9) CSE chunk 950/1341 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part34_v4rho3sigma_9_chunk950<F: Float>(t225: F, t22917: F, t22923: F, t22927: F, t22933: F, t22813: F, t9880: F, t5651: F, t6816: F, t1394: F, t22809: F, t1877: F, t1879: F, t539: F, t541: F, t5650: F, t6832: F, t6837: F, t6840: F) -> F {
    let t22936 = (t22917 + t22923 + t22927 + t22933) * t225;
    let t22944 = t9880 * t22813;
    let t22947 = t5651 * t6816;
    let t22950 = t1394 * t22809;
    let t22953 = -F::new(36.0) * t1877 * t6837 + F::new(9.0) * t1877 * t6840 + F::new(9.0) * t1879 * t6832 - t22936 * t541 + F::new(60.0) * t22944 * t539 - F::new(36.0) * t22947 * t5650 + F::new(3.0) * t22950 * t539;
    t22953
}
