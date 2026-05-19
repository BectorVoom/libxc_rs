//! GGA_C_ACGGAP lxc pol — lxc_pol part 13 (v4rho3sigma_5) CSE chunk 869/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part13_v4rho3sigma_5_chunk869<F: Float>(t30238: F, t1973: F, t7610: F, t1985: F, t30196: F, t3668: F, t587: F, t381: F, t390: F, t151: F) -> (F, F, F, F, F) {
    let t30239 = F::cast_from(0.10718504529517434243e-3_f64) * t30238;
    let t30240 = t7610 * t1973;
    let t30242 = t30196 * t1985;
    let t30243 = F::cast_from(0.21437009059034868486e-3_f64) * t30242;
    let t30244 = t587 * t3668;
    let t30246 = t381 * t30244 * t390;
    let t30247 = F::cast_from(0.34013387707001991332e-1_f64) * t30246;
    let t30248 = t151 * t30244;
    (t30239, t30240, t30243, t30247, t30248)
}
