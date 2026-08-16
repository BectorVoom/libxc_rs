//! MGGA_C_R2SCAN lxc pol — lxc_pol part 15 (v4rho3sigma_5) CSE chunk 966/1253 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part15_v4rho3sigma_5_chunk966<F: Float>(t1065: F, t792: F, t11002: F, t3269: F, t1102: F, t3314: F, t3457: F, t2333: F, t481: F, t795: F, t3263: F, t3262: F) -> (F, F, F, F, F, F) {
    let t11003 = t1065 * t792;
    let t11004 = t11002 * t11003;
    let t11005 = t3269 * t11004;
    let t11006 = F::cast_from(5.0_f64) / F::cast_from(8.0_f64) * t11005;
    let t11008 = t1102 * t3314 * t3457;
    let t11010 = t2333 * t481;
    let t11011 = t11010 * t795;
    let t11012 = t3263 * t11011;
    let t11013 = t3262 * t11012;
    (t11004, t11006, t11008, t11011, t11012, t11013)
}
