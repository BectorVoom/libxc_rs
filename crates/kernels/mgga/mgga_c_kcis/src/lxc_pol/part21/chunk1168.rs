//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1168/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk1168<F: Float>(t28140: F, t28174: F, t28222: F, t28248: F, t2205: F, t5394: F, t11223: F, t15109: F, t27141: F, t27986: F, t27988: F, t27989: F, t27992: F, t27998: F, t28001: F, t28008: F, t28073: F, t28076: F, t3669: F, t437: F, t5363: F, t7809: F, t8108: F) -> (F, F, F) {
    let t28250 = t28140 + t28174 + t28222 + t28248;
    let t28253 = t2205 * t5394;
    let t28256 = F::cast_from(2.0_f64) * t11223 * t8108 - t15109 * t2205 + F::cast_from(2.0_f64) * t27141 * t5363 + F::cast_from(2.0_f64) * t28073 * t3669 + F::cast_from(2.0_f64) * t28076 * t3669 + t28250 * t437 + F::cast_from(2.0_f64) * t28253 * t3669 - t5394 * t7809 - t27986 + t27988 + t27989 + t27992 - t27998 + t28001 + t28008;
    (t28250, t28253, t28256)
}
