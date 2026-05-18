//! GGA_C_ACGGAP lxc pol — lxc_pol part 11 (v4rho3sigma_3) CSE chunk 833/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part11_v4rho3sigma_3_chunk833<F: Float>(t1603: F, t615: F, t2331: F, t315: F, t323: F, t557: F, t7973: F, t2341: F, t322: F, t2147: F, t2138: F, t2347: F, t621: F, t7912: F, t7962: F, t7967: F, t7974: F, t7977: F, t7981: F, t7985: F, t7988: F, t7991: F, t7996: F, t8000: F) -> (F, F, F, F) {
    let t9058 = t615 * t1603;
    let t9062 = t315 * t2331;
    let t9063 = t9062 * t323;
    let t9073 = t7973 * t557;
    let t9075 = t2341 * t322;
    let t9076 = t2147 * t9075;
    let t9077 = t2138 * t9076;
    let t9079 = t7962 - F::new(0.4336814094102599731e0) * t9058 * t621 + F::new(0.8673628188205199462e0) * t7967 - F::new(0.65854491829355115987e0) * t9063 - F::new(0.65854491829355115987e0) * t7974 - F::new(0.65854491829355115987e0) * t7977 + F::new(0.4336814094102599731e0) * t7912 * t2347 - F::new(0.8673628188205199462e0) * t7981 + F::new(0.8673628188205199462e0) * t7985 - F::new(0.8673628188205199462e0) * t7988 + F::new(0.8673628188205199462e0) * t7991 + t7996 - t8000 - F::new(0.65854491829355115987e0) * t9073 - F::new(0.17347256376410398924e1) * t9077;
    (t9058, t9062, t9076, t9079)
}
