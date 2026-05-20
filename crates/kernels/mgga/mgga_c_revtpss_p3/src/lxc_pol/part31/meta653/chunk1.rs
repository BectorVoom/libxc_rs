//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 2177/2259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2177<F: Float>(t106596: F, t107882: F, t107885: F, t107892: F, t107895: F, t107901: F, t107908: F, t107919: F, t1940: F, t1963: F, t20256: F, t2403: F, t25206: F, t27770: F, t27793: F, t27800: F, t29939: F, t29949: F, t29953: F, t4541: F, t7087: F, t98637: F) -> F {
    let t107922 = -F::new(3.0) / F::new(2.0) * t25206 * t107882 - F::new(3.0) * t25206 * t107885 + F::new(2.0) * t106596 * t27800 - F::new(3.0) * t98637 * t27793 - F::new(3.0) * t25206 * t107892 - F::new(3.0) * t25206 * t107895 + F::new(3.0) * t2403 * t7087 * t29949 + F::new(3.0) * t2403 * t1963 * t107901 + t1940 * t1963 * t20256 / F::new(2.0) + F::new(3.0) * t25206 * t107908 + F::new(3.0) / F::new(2.0) * t2403 * t7087 * t29953 + F::new(3.0) * t4541 * t7087 * t29939 - F::new(3.0) * t98637 * t27770 - F::new(3.0) / F::new(2.0) * t25206 * t107919;
    t107922
}
