//! GGA_C_GAPLOC lxc pol — lxc_pol part 39 (v4rhosigma3_4) CSE chunk 908/1217 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part39_v4rhosigma3_4_chunk908<F: Float>(t41676: F, t2875: F, t4386: F, t544: F, t9078: F, t2792: F, t3177: F, t9263: F, t9278: F, t993: F, t20671: F, t31041: F, t34818: F) -> (F, F, F, F, F) {
    let t41677 = F::cast_from(0.29792074959875355558e-1_f64) * t41676;
    let t41681 = F::cast_from(0.27805936629216998521e0_f64) * t544 * t9078 * t2875 * t4386;
    let t41683 = t9263 * t2792 * t3177;
    let t41684 = F::cast_from(0.76685851907841499353e0_f64) * t41683;
    let t41686 = t9263 * t993 * t9278;
    let t41687 = F::cast_from(0.76685851907841499353e0_f64) * t41686;
    let t41689 = t31041 * t20671 * t34818;
    (t41677, t41681, t41684, t41687, t41689)
}
