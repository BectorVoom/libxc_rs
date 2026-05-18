//! MGGA_C_REVTPSS lxc pol — lxc_pol part 26 (v4rho3sigma_1) CSE chunk 696/1225 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part26_v4rho3sigma_1_chunk696<F: Float>(t1956: F, t2067: F, t213: F, t257: F, t7067: F, t7070: F, t7387: F, t7390: F, t7399: F, t7403: F, t7409: F, t7411: F, t7415: F, t7420: F, t7424: F, t887: F) -> F {
    let t7427 = -t7387 + t7390 + F::new(0.65854491829355115987e0) * t213 * t7399 * t257 - F::new(0.65854491829355115987e0) * t7403 * t887 + t7409 - t7411 - F::new(0.4336814094102599731e0) * t7067 * t2067 + F::new(0.8673628188205199462e0) * t7070 * t7415 + F::new(0.4336814094102599731e0) * t7070 * t7420 - F::new(0.4336814094102599731e0) * t1956 * t7424;
    t7427
}
