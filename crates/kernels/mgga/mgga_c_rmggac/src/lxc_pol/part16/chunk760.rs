//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 760/1012 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk760<F: Float>(t40965: F, t8620: F, t22: F, t235: F, t34812: F, t1982: F, t2314: F, t35512: F, t2289: F, t7921: F, t6355: F, t7707: F, t1550: F, t41548: F, t1978: F, t7228: F, t8511: F) -> (F, F, F, F, F, F, F) {
    let t41735 = t8620 * t40965;
    let t41738 = t235 * t34812 * t22;
    let t41767 = t2314 * t35512 * t1982;
    let t41774 = t7921 * t2289;
    let t41789 = t6355 * t7707;
    let t41791 = t1550 * t41548;
    let t41799 = t8511 * t7228 * t1978;
    (t41735, t41738, t41767, t41774, t41789, t41791, t41799)
}
