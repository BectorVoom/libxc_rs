//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 2045/2259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2045<F: Float>(t1889: F, t94545: F, t13846: F, t13877: F, t7021: F, t5665: F, t94497: F, t14036: F, t25997: F, t13941: F, t94423: F, t14005: F) -> (F, F, F, F, F, F) {
    let t98165 = t94545 * t1889;
    let t98168 = t7021 * t13846 * t13877;
    let t98169 = F::new(7.0) / F::new(24.0) * t98168;
    let t98174 = t94497 * t5665;
    let t98180 = t25997 * t14036;
    let t98181 = F::cast_from(0.50820002809285328226e-4_f64) * t98180;
    let t98185 = t94423 * t13941;
    let t98186 = F::cast_from(0.2032800112371413129e-3_f64) * t98185;
    let t98187 = t94423 * t14005;
    (t98165, t98169, t98174, t98181, t98186, t98187)
}
