//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 991/1127 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk991<F: Float>(t34764: F, t8457: F, t16503: F, t16504: F, t571: F, t7467: F, t3369: F, t7482: F, t34975: F, t35039: F, t38649: F, t495: F, t8440: F) -> (F, F, F, F) {
    let t41747 = t34764 * t8457;
    let t41751 = t16503 * t16504 * t571 * t7467;
    let t41755 = t16503 * t3369 * t571 * t7482;
    let t41760 = t34975 * t35039 * t8440 * t38649 * t495;
    (t41747, t41751, t41755, t41760)
}
