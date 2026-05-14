//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 983/1141 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk983<F: Float>(t41046: F, t41048: F, t10465: F, t12509: F, t5211: F, t41053: F, t47782: F, t47784: F, t47786: F, t47790: F, t47793: F, t47794: F, t47795: F, t47800: F, t41056: F, t41061: F) -> (F, F, F, F, F, F, F) {
    let t47801 = 32.0 / 15.0 * t41046;
    let t47802 = 32.0 / 15.0 * t41048;
    let t47805 = 64.0 / 15.0 * t5211 * t10465 * t12509;
    let t47806 = 32.0 / 45.0 * t41053;
    let t47807 = t47782 - t47784 - t47786 - t47790 + t47793 + t47794 - t47795 + t47800 + t47801 + t47802 - t47805 + t47806;
    let t47809 = 32.0 / 135.0 * t41056;
    let t47810 = 64.0 / 15.0 * t41061;
    (t47801, t47802, t47805, t47806, t47807, t47809, t47810)
}
