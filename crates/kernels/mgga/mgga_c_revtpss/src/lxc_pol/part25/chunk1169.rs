//! MGGA_C_REVTPSS lxc pol — lxc_pol part 25 (v4rho3sigma_0) CSE chunk 1169/1360 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part25_v4rho3sigma_0_chunk1169<F: Float>(t1312: F, t25832: F, t2371: F, t25096: F, t25169: F, t25805: F, t25812: F, t25814: F, t25816: F, t25818: F, t25820: F, t670: F, t6985: F) -> F {
    let t25834 = F::new(2.0) * t1312 * t25832;
    let t25835 = F::new(2.0) * t2371 * t6985 + F::new(4.0) * t25805 * t670 + F::new(2.0) * t25096 + t25169 + t25812 + t25814 + t25816 + t25818 + t25820 + t25834;
    t25835
}
