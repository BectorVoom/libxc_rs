//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 432/1340 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk432<F: Float>(t1830: F, t709: F, t713: F, t257: F, t712: F) -> (F, F, F, F) {
    let t1944 = F::new(0.12361111111111111111e-1) * t1830;
    let t1950 = t709 * t713;
    let t1953 = t712 * t257;
    let t1954 = F::new(1.0) / t1953;
    (t1944, t1950, t1953, t1954)
}
