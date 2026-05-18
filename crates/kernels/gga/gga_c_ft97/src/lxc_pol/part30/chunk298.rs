//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 298/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk298<F: Float>(t1197: F, t4061: F, t1196: F, t816: F, t820: F, t1095: F, t2697: F, t274: F, t688: F, t3750: F, t801: F, t231: F) -> (F, F, F, F, F, F, F) {
    let t4062 = t4061 * t1197;
    let t4064 = t816 * t1196;
    let t4065 = t4064 * t820;
    let t4068 = t2697 * t1095;
    let t4069 = t274 * t688;
    let t4072 = t801 * t3750;
    let t4073 = t4072 * t274;
    let t4075 = t1095 * t688;
    let t4077 = t231 * t4075 * t274;
    (t4062, t4065, t4068, t4069, t4072, t4073, t4077)
}
