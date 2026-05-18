//! GGA_C_FT97 lxc pol — lxc_pol part 3 (v3rho3_2) CSE chunk 693/1032 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part3_v3rho3_2_chunk693<F: Float>(t100: F, t1587: F, t487: F, t942: F, t1882: F, t3231: F, t3201: F, t8392: F, t3170: F, t8232: F, t955: F, t3227: F) -> (F, F, F, F, F, F, F) {
    let t11810 = t1587 * t100;
    let t11811 = t487 * t942;
    let t11821 = F::new(2.0) / F::new(9.0) * t1882 * t3231;
    let t11826 = F::new(2.0) / F::new(27.0) * t8392 * t3201;
    let t11837 = t3170 * t487;
    let t11846 = t8232 * t955;
    let t11849 = F::new(2.0) / F::new(9.0) * t1882 * t3227;
    (t11810, t11811, t11821, t11826, t11837, t11846, t11849)
}
