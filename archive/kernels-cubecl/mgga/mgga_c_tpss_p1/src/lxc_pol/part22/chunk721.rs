//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 721/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk721<F: Float>(t45: F, t3565: F, t581: F, t3564: F, t190: F, t3431: F, t681: F, t1351: F, t680: F, t682: F, t1289: F, t2225: F, t78: F, zeta_threshold: F) -> (F, F, F, F, F, F, F, F) {
    let t151 = t45 <= zeta_threshold;
    let t3566 = t3565 * t581;
    let t3568 = F::cast_from(12.0_f64) * t3564 * t3566;
    let t3569 = t190 * t3431;
    let t3571 = F::cast_from(4.0_f64) * t681 * t3569;
    let t3572 = t680 * t1351;
    let t3574 = F::cast_from(4.0_f64) * t3572 * t682;
    let t3575 = t2225 * t1289;
    let t3581 = piecewise3::<F>(t151, F::cast_from(0.0_f64), F::cast_from(4.0_f64) / F::cast_from(9.0_f64) * t3575 * t581 + F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t78 * t3431);
    (t3566, t3568, t3569, t3571, t3572, t3574, t3575, t3581)
}
