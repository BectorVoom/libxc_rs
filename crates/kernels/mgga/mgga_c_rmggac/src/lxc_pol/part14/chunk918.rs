//! MGGA_C_RMGGAC lxc pol — lxc_pol part 14 (v4rho3sigma_5) CSE chunk 918/952 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part14_v4rho3sigma_5_chunk918<F: Float>(t34975: F, t35039: F, t38649: F, t495: F, t8440: F, t275: F, t8887: F, t1982: F, t2314: F, t35512: F, t118: F, t128: F, t2001: F, t5738: F, t675: F, t2289: F, t7921: F) -> (F, F, F, F, F) {
    let t41760 = t34975 * t35039 * t8440 * t38649 * t495;
    let t41763 = 2.0 * t275 * t8887;
    let t41767 = t2314 * t35512 * t1982;
    let t41772 = t675 * t2001 * t118 * t128 * t5738;
    let t41774 = t7921 * t2289;
    (t41760, t41763, t41767, t41772, t41774)
}
