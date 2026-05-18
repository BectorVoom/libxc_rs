//! GGA_C_FT97 lxc pol — lxc_pol part 3 (v3rho3_2) CSE chunk 718/1032 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part3_v3rho3_2_chunk718<F: Float>(t13794: F, t1882: F, t3696: F, t3701: F, t3951: F, t761: F, t1160: F, t737: F, t1144: F, t8232: F, t3991: F, t3899: F, t8392: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t13795 = F::new(4.0) / F::new(81.0) * t13794;
    let t13809 = t1882 * t3696;
    let t13810 = F::new(2.0) / F::new(27.0) * t13809;
    let t13811 = t1882 * t3701;
    let t13812 = F::new(4.0) / F::new(27.0) * t13811;
    let t13830 = t3951 * t761;
    let t13839 = t737 * t1160;
    let t13872 = t8232 * t1144;
    let t13875 = F::new(2.0) / F::new(9.0) * t1882 * t3991;
    let t13884 = F::new(2.0) / F::new(27.0) * t8392 * t3899;
    (t13795, t13809, t13810, t13811, t13812, t13830, t13839, t13872, t13875, t13884)
}
