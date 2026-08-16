//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 917/1242 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk917<F: Float>(t17022: F, t1809: F, t639: F, t5137: F, t5145: F, t1407: F, t1820: F, t1821: F, t4886: F, t1672: F, t1805: F, t185: F) -> (F, F, F, F) {
    let t17222 = F::cast_from(16.0_f64) / F::cast_from(5.0_f64) * t639 * t1809 * t17022;
    let t17224 = t639 * t5137 * t5145;
    let t17225 = F::cast_from(32.0_f64) / F::cast_from(45.0_f64) * t17224;
    let t17229 = F::cast_from(16.0_f64) / F::cast_from(15.0_f64) * t1820 * t1821 * t4886 * t1407;
    let t17231 = t185 * t1672 * t1805;
    (t17222, t17225, t17229, t17231)
}
