//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 716/988 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk716<F: Float>(t2004: F, t7921: F, t2007: F, t1987: F, t1990: F, t1993: F, t7920: F, t1997: F, t7335: F, t7927: F, t16156: F, t7742: F, t7380: F, t5542: F, t7546: F, t674: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t36508 = t7921 * t2004;
    let t36511 = t7921 * t2007;
    let t36513 = t7921 * t1987;
    let t36515 = t7921 * t1990;
    let t36520 = t1993 * t7920;
    let t36521 = t36520 * t1997;
    let t36527 = t7335 * t7927;
    let t36533 = t16156 * t7742;
    let t36535 = t16156 * t7380;
    let t36541 = t7546 * t5542;
    let t36542 = t36541 * t674;
    (t36508, t36511, t36513, t36515, t36520, t36521, t36527, t36533, t36535, t36541, t36542)
}
