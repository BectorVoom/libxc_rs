//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 712/1030 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk712<F: Float>(t10478: F, t309: F, t2349: F, t824: F, t4140: F, t2347: F, t870: F, t875: F, t4139: F, t2680: F, t665: F) -> (F, F, F, F, F, F, F, F, F) {
    let t10479 = t10478 * t309;
    let t10480 = t2349 * t824;
    let t10481 = t4140 * t10480;
    let t10482 = t10479 * t10481;
    let t10485 = t870 * t2347;
    let t10486 = t2349 * t875;
    let t10487 = t10485 * t10486;
    let t10488 = t4139 * t10487;
    let t10491 = t665 * t2680;
    (t10479, t10480, t10481, t10482, t10485, t10486, t10487, t10488, t10491)
}
