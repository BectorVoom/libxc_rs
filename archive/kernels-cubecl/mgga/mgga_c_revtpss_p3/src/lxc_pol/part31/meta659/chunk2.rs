//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 2232/2259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2232<F: Float>(t5: F, t108768: F, t108799: F, t108829: F, t108854: F, t108889: F, t108931: F, t108963: F, t109001: F, t117: F, t27154: F, t98450: F, t28177: F, t7898: F) -> (F, F, F) {
    let t7 = piecewise3::<F>(F::cast_from(0.0_f64) < t5, t5, -t5);
    let t8 = -t7 <= -F::cast_from(0.999999999999e0_f64);
    let t109005 = piecewise3::<F>(t8, F::cast_from(0.0_f64), t108768 + t108799 + t108829 + t108854 + t108889 + t108931 + t108963 + t109001);
    let t109006 = t109005 * t117;
    let t109012 = F::cast_from(6.0_f64) * t98450 * t27154;
    let t109014 = F::cast_from(6.0_f64) * t7898 * t28177;
    (t109006, t109012, t109014)
}
