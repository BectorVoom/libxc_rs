//! MGGA_C_KCISK lxc pol — lxc_pol part 26 (v4rho3sigma_6) CSE chunk 1210/1407 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part26_v4rho3sigma_6_chunk1210<F: Float>(t35047: F, t2752: F, t7728: F, t294: F, t2071: F, t9895: F, t296: F, t8459: F, t2709: F, t2707: F, t7724: F, t559: F, t7727: F, t2070: F, t2351: F, t1319: F, t13440: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t35048 = t35047 / 16.0;
    let t35050 = t7728 * t2752;
    let t35051 = t294 * t35050;
    let t35052 = t35051 / 16.0;
    let t35053 = t2071 * t9895;
    let t35054 = t294 * t35053;
    let t35055 = t35054 / 8.0;
    let t35056 = t296 * t8459;
    let t35057 = t2709 * t35056;
    let t35058 = t35057 / 16.0;
    let t35059 = t7724 * t2707;
    let t35060 = t7727 * t559;
    let t35061 = t2709 * t35060;
    let t35062 = t35061 / 16.0;
    let t35063 = t2070 * t2351;
    let t35064 = t2709 * t35063;
    let t35065 = t35064 / 8.0;
    let t35843 = t1319 * t13440;
    (t35048, t35050, t35052, t35053, t35055, t35056, t35058, t35059, t35060, t35062, t35063, t35065, t35843)
}
