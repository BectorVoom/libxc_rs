//! MGGA_C_REVTPSS lxc pol — lxc_pol part 53 (v4rho2sigma2_8) CSE chunk 671/1244 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part53_v4rho2sigma2_8_chunk671<F: Float>(t1949: F, t231: F, t836: F, t7076: F, t233: F, t7048: F, t1957: F, t1956: F, t1959: F, t213: F, t257: F, t7017: F, t7020: F, t7049: F, t7053: F, t7062: F, t7066: F, t7067: F, t7070: F, t7073: F, t887: F) -> (F, F, F, F) {
    let t7078 = t1949 * t836 * t231;
    let t7079 = t7076 * t7078;
    let t7082 = t233 * t7048;
    let t7083 = t1957 * t7082;
    let t7086 = -t7017 + t7020 + F::new(0.65854491829355115987e0) * t213 * t7049 * t257 - F::new(0.65854491829355115987e0) * t7053 * t887 + t7062 - t7066 - F::new(0.4336814094102599731e0) * t7067 * t1959 + F::new(0.8673628188205199462e0) * t7070 * t7073 + F::new(0.4336814094102599731e0) * t7070 * t7079 - F::new(0.4336814094102599731e0) * t1956 * t7083;
    (t7079, t7082, t7083, t7086)
}
