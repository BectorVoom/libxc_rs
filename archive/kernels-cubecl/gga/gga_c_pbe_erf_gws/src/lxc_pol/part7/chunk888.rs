//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 888/1242 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk888<F: Float>(t16883: F, t1663: F, t1820: F, t1821: F, t4352: F, t562: F, t1680: F, t1740: F, t5516: F, t612: F, t5125: F, t5133: F) -> (F, F, F, F, F) {
    let t16884 = F::cast_from(128.0_f64) / F::cast_from(45.0_f64) * t16883;
    let t16889 = F::cast_from(64.0_f64) / F::cast_from(15.0_f64) * t1820 * t1821 * t562 * t1663 * t4352;
    let t16890 = t1680 * t1740;
    let t16891 = F::cast_from(32.0_f64) / F::cast_from(15.0_f64) * t16890;
    let t16893 = F::cast_from(16.0_f64) / F::cast_from(5.0_f64) * t5516 * t612;
    let t16895 = t1820 * t5125 * t5133;
    (t16884, t16889, t16891, t16893, t16895)
}
