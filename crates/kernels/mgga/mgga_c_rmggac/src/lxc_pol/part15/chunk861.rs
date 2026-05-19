//! MGGA_C_RMGGAC lxc pol — lxc_pol part 15 (v4rho3sigma_6) CSE chunk 861/1110 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part15_v4rho3sigma_6_chunk861<F: Float>(t40951: F, t38187: F, t38188: F, t38191: F, t38192: F, t38193: F, t38194: F, t8350: F, t8356: F, t9722: F, t9725: F, t9728: F) -> (F, F) {
    let t44089 = F::cast_from(0.2927036860455597649e0_f64) * t40951;
    let t44512 = -t9722 - t9725 - t9728 + t38187 - t38188 - t38191 - F::cast_from(0.60975299583150056628e-3_f64) * t8350 - t38192 - F::cast_from(0.60975299583150056628e-3_f64) * t8356 - t38193 - t38194;
    (t44089, t44512)
}
