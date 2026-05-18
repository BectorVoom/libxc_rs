//! MGGA_C_RMGGAC lxc pol — lxc_pol part 15 (v4rho3sigma_6) CSE chunk 851/1110 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part15_v4rho3sigma_6_chunk851<F: Float>(t36733: F, t8450: F, t7478: F, t7244: F, t8432: F, t1614: F, t2084: F, t2139: F, t27: F, t34884: F, t9123: F, t4601: F, t9008: F) -> (F, F, F, F, F, F) {
    let t42085 = t8450 * t36733;
    let t42086 = t42085 * t7478;
    let t42087 = F::new(0.19863479950205658386e-4) * t42086;
    let t42101 = t7244 * t8432;
    let t42132 = t2139 * t27 * t2084 * t1614;
    let t42144 = t34884 * t9123;
    let t42145 = F::new(0.24829349937757072982e-4) * t42144;
    let t42151 = t4601 * t9008;
    (t42085, t42087, t42101, t42132, t42145, t42151)
}
