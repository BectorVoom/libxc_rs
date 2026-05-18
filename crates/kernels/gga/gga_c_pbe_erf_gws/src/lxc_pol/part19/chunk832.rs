//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 832/1404 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk832<F: Float>(t4358: F, t2626: F, t5018: F, t1820: F, t1648: F, t2643: F, t2602: F, t5493: F, t639: F, t2631: F, t587: F, t589: F, t837: F) -> (F, F, F, F, F, F) {
    let t7907 = F::new(12.0) * t4358;
    let t7913 = t5018 * t2626;
    let t7915 = F::new(16.0) / F::new(45.0) * t1820 * t7913;
    let t7919 = F::new(16.0) / F::new(135.0) * t1648 * t2643;
    let t7925 = t5493 * t2602;
    let t7927 = F::new(16.0) / F::new(45.0) * t639 * t7925;
    let t7932 = t5018 * t2631;
    let t7934 = F::new(16.0) / F::new(45.0) * t587 * t7932;
    let t7940 = t837 * t589;
    (t7907, t7915, t7919, t7927, t7934, t7940)
}
