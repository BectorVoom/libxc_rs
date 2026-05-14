//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 1058/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk1058<F: Float>(t31751: F, t31802: F, t31851: F, t31950: F, t312: F, t31930: F, t2: F, t21930: F, t4: F, t26: F) -> (F, F, F, F) {
    let t31952 = t31751 + t31802 + t31851 + t31950;
    let t31956 = t31930 * t312;
    let t31961 = t21930 * t2;
    let t31962 = t31961 * t4;
    let t31963 = t31962 * t26;
    (t31952, t31956, t31962, t31963)
}
