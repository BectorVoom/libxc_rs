//! MGGA_C_RMGGAC lxc pol — lxc_pol part 15 (v4rho3sigma_6) CSE chunk 843/1110 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part15_v4rho3sigma_6_chunk843<F: Float>(t40898: F, t7198: F, t1664: F, t2127: F, t16156: F, t9055: F, t2085: F, t8339: F, t1692: F, t2046: F, t2050: F, t31: F) -> (F, F, F, F, F) {
    let t41641 = t7198 * t40898;
    let t41651 = t1664 * t2127;
    let t41654 = t16156 * t9055;
    let t41656 = t8339 * t2085;
    let t41657 = F::new(0.18183107769496894486e-1) * t41656;
    let t41667 = t2046 * t2050 * t1692 * t31;
    (t41641, t41651, t41654, t41657, t41667)
}
