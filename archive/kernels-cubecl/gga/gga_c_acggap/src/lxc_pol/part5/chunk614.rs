//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 614/1332 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk614<F: Float>(t3037: F, t317: F, t3922: F, t180: F, t3242: F, t323: F, t868: F, t880: F, t3054: F, t865: F, t191: F, t813: F) -> (F, F, F, F, F, F, F, F) {
    let t3923 = t317 * t3037;
    let t3925 = F::cast_from(0.39512695097613069591e1_f64) * t3922 * t3923;
    let t3930 = t3242 * t180;
    let t3932 = F::cast_from(0.19756347548806534796e1_f64) * t3930 * t323;
    let t3935 = t868 * t880;
    let t3937 = t3054 * t180;
    let t3939 = F::cast_from(0.39512695097613069591e1_f64) * t3937 * t865;
    let t3952 = F::cast_from(1.0_f64) / t813 / t191;
    (t3923, t3925, t3930, t3932, t3935, t3937, t3939, t3952)
}
