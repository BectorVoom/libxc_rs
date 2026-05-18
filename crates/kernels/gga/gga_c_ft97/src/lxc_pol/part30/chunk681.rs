//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 681/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk681<F: Float>(t25188: F, t4181: F, t7124: F, t875: F, t2843: F, t1248: F, t25253: F, t7091: F, t870: F) -> (F, F, F, F, F) {
    let t28852 = t25188 * t4181;
    let t28854 = t7124 * t875;
    let t28855 = t2843 * t28854;
    let t28857 = t25253 * t1248;
    let t28859 = t7091 * t870;
    (t28852, t28854, t28855, t28857, t28859)
}
