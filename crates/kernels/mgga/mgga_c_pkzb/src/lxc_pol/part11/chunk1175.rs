//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1175/1340 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1175<F: Float>(t28889: F, t28910: F, t98: F, t126: F, t83: F, t10534: F, t545: F, t19620: F, t19625: F, t19627: F, t16476: F, t16193: F, t16230: F, t16273: F, t16275: F, t16280: F, t16283: F, t16287: F, t16290: F, t16481: F, t16486: F, t16489: F, t19624: F, t19688: F, t19690: F) -> (F, F, F, F, F, F, F, F) {
    let t28912 = (t28889 + t28910) * t98;
    let t28914 = t83 * t28912 * t126;
    let t28916 = t83 * t10534 * t545;
    let t28917 = F::new(0.17090684152272775384e-2) * t19620;
    let t28918 = F::new(0.48796115851357829289e-1) * t19625;
    let t28919 = F::new(0.14447919941302971323e1) * t19627;
    let t28920 = F::new(0.35089341735807877242e1) * t16476;
    let t28921 = -t16193 + t28914 + t28916 - t16230 - t16273 + t16275 - t28917 + t19624 + t28918 + t28919 - t16280 + t16283 + t16287 - t16290 + t19688 + t19690 + t28920 + t16481 - t16486 - t16489;
    (t28912, t28914, t28916, t28917, t28918, t28919, t28920, t28921)
}
