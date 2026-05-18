//! MGGA_C_RMGGAC lxc pol — lxc_pol part 14 (v4rho3sigma_5) CSE chunk 946/1089 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part14_v4rho3sigma_5_chunk946<F: Float>(t36343: F, t8457: F, t1981: F, t3142: F, t508: F, t8512: F, t1652: F, t2084: F, t2145: F, t27: F, t16156: F, t9213: F) -> (F, F, F, F) {
    let t40250 = t36343 * t8457;
    let t40251 = F::new(0.24829349937757072982e-4) * t40250;
    let t40254 = t8512 * t1981 * t3142 * t508;
    let t40259 = t2145 * t27 * t2084 * t1652;
    let t40260 = F::new(0.18183107769496894486e-1) * t40259;
    let t40262 = t16156 * t9213;
    (t40251, t40254, t40260, t40262)
}
