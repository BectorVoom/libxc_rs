//! MGGA_C_RMGGAC lxc pol — lxc_pol part 15 (v4rho3sigma_6) CSE chunk 651/1110 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part15_v4rho3sigma_6_chunk651<F: Float>(t3351: F, t9051: F, t5144: F, t515: F, t3352: F, t2028: F, t2868: F, t9008: F, t903: F, t1550: F, t9000: F, t1685: F, t668: F) -> (F, F, F, F, F, F, F) {
    let t9052 = t3351 * t9051;
    let t9054 = t515 * t5144;
    let t9055 = t3352 * t9054;
    let t9056 = t3351 * t9055;
    let t9058 = t2868 * t2028;
    let t9060 = t903 * t9008;
    let t9062 = t1550 * t9000;
    let t9064 = t1685 * t668;
    (t9052, t9055, t9056, t9058, t9060, t9062, t9064)
}
