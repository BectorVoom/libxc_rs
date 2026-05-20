//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 3078/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3078<F: Float>(t12228: F, t1732: F, t44091: F, t44093: F, t43748: F, t5068: F, t45046: F, t5109: F, t12361: F, t16652: F, t12243: F, t16662: F) -> (F, F, F, F, F) {
    let t56275 = F::cast_from(0.24955700379505800916e5_f64) * t44091 * t1732 * t44093 * t12228;
    let t56277 = F::new(6.0) * t43748 * t5068;
    let t56279 = F::cast_from(0.48245938496077605201e2_f64) * t45046 * t5109;
    let t56281 = F::new(12.0) * t12361 * t16652;
    let t56283 = F::cast_from(0.96491876992155210402e2_f64) * t12243 * t16662;
    (t56275, t56277, t56279, t56281, t56283)
}
