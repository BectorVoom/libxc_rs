//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 958/1242 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk958<F: Float>(t17689: F, t17713: F, t17734: F, t17765: F, t185: F, t186: F, t598: F, t4920: F, t5312: F, t1403: F, t1406: F, t1764: F, t1820: F, t1821: F) -> (F, F, F) {
    let t17771 = F::new(2.0) / F::new(15.0) * t185 * t186 * t598 * (t17689 + t17713 + t17734 + t17765);
    let t17773 = F::new(64.0) / F::new(15.0) * t5312 * t4920;
    let t17778 = F::new(32.0) / F::new(15.0) * t1820 * t1821 * t1406 * t1764 * t1403;
    (t17771, t17773, t17778)
}
