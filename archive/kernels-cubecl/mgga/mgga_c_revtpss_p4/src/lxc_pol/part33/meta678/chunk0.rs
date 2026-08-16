//! MGGA_C_REVTPSS lxc pol — lxc_pol part 33 (v4rho3sigma_8) CSE chunk 2210/2275 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2210<F: Float>(t108710: F, t1936: F, t21881: F, t93: F, t30143: F, t7002: F, t27123: F, t7741: F, t28219: F, t28042: F, t7889: F, t2322: F, t30004: F) -> (F, F, F, F, F, F, F) {
    let t109241 = F::cast_from(2.0_f64) * t108710 * t1936;
    let t109242 = t93 * t21881;
    let t109244 = F::cast_from(2.0_f64) * t109242 * t1936;
    let t109246 = F::cast_from(2.0_f64) * t30143 * t7002;
    let t109248 = F::cast_from(4.0_f64) * t27123 * t7741;
    let t109250 = F::cast_from(4.0_f64) * t28219 * t7741;
    let t109252 = F::cast_from(4.0_f64) * t7889 * t28042;
    let t109254 = F::cast_from(2.0_f64) * t2322 * t30004;
    (t109241, t109244, t109246, t109248, t109250, t109252, t109254)
}
