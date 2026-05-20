//! MGGA_C_REVTPSS lxc pol — lxc_pol part 28 (v4rho3sigma_3) CSE chunk 1228/2277 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1228<F: Float>(t7076: F, t7774: F, t233: F, t7759: F, t1957: F, t1580: F, t1956: F, t1959: F, t213: F, t257: F, t7017: F, t7020: F, t7053: F, t7062: F, t7066: F, t7070: F, t7760: F, t7766: F, t7770: F) -> (F, F, F, F) {
    let t7775 = t7076 * t7774;
    let t7778 = t233 * t7759;
    let t7779 = t1957 * t7778;
    let t7782 = -t7017 + t7020 + F::cast_from(0.65854491829355115987e0_f64) * t213 * t7760 * t257 - F::cast_from(0.65854491829355115987e0_f64) * t7053 * t1580 + t7062 - t7066 - F::cast_from(0.4336814094102599731e0_f64) * t7766 * t1959 + F::cast_from(0.8673628188205199462e0_f64) * t7070 * t7770 + F::cast_from(0.4336814094102599731e0_f64) * t7070 * t7775 - F::cast_from(0.4336814094102599731e0_f64) * t1956 * t7779;
    (t7775, t7778, t7779, t7782)
}
