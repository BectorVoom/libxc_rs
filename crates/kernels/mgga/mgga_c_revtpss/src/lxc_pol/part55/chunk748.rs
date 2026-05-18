//! MGGA_C_REVTPSS lxc pol — lxc_pol part 55 (v4rho2sigma2_10) CSE chunk 748/1306 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part55_v4rho2sigma2_10_chunk748<F: Float>(t1580: F, t1956: F, t1959: F, t213: F, t257: F, t7017: F, t7020: F, t7053: F, t7062: F, t7066: F, t7070: F, t7760: F, t7766: F, t7770: F, t7775: F, t7779: F) -> F {
    let t7782 = -t7017 + t7020 + F::new(0.65854491829355115987e0) * t213 * t7760 * t257 - F::new(0.65854491829355115987e0) * t7053 * t1580 + t7062 - t7066 - F::new(0.4336814094102599731e0) * t7766 * t1959 + F::new(0.8673628188205199462e0) * t7070 * t7770 + F::new(0.4336814094102599731e0) * t7070 * t7775 - F::new(0.4336814094102599731e0) * t1956 * t7779;
    t7782
}
