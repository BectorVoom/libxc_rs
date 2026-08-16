//! GGA_C_FT97 lxc pol — lxc_pol part 3 (v3rho3_2) CSE chunk 521/1032 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part3_v3rho3_2_chunk521<F: Float>(t1196: F, t816: F, t820: F, t1095: F, t2697: F, t274: F, t688: F, t3750: F, t801: F, t231: F, t1193: F, t278: F) -> (F, F, F, F, F, F, F, F, F) {
    let t4064 = t816 * t1196;
    let t4065 = t4064 * t820;
    let t4068 = t2697 * t1095;
    let t4069 = t274 * t688;
    let t4072 = t801 * t3750;
    let t4073 = t4072 * t274;
    let t4075 = t1095 * t688;
    let t4077 = t231 * t4075 * t274;
    let t4080 = t1193 * t688;
    let t4083 = t3750 * t278;
    (t4064, t4065, t4068, t4069, t4072, t4073, t4077, t4080, t4083)
}
