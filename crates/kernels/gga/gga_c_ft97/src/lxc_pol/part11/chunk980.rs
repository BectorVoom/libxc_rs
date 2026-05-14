//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 980/1030 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk980<F: Float>(t2956: F, t10818: F, t5: F, t2832: F, t2842: F, t2844: F, t2801: F, t2843: F, t10799: F, t875: F, t2857: F, t41473: F, t446: F, t2404: F, t2680: F, t2405: F, t2682: F) -> (F, F, F, F, F, F, F, F) {
    let t43304 = t2956 * t2956;
    let t43311 = t5 * t10818;
    let t43328 = t2832 * t2842;
    let t43329 = t43328 * t2844;
    let t43331 = t2801 * t2801;
    let t43332 = t2843 * t43331;
    let t43335 = t2843 * t875 * t10799;
    let t43348 = t446 * t2857 * t41473;
    let t43350 = t2404 * t2680;
    let t43351 = t2405 * t2682;
    (t43304, t43311, t43329, t43332, t43335, t43348, t43350, t43351)
}
