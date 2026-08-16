//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 651/1242 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk651<F: Float>(t1823: F, t5125: F, t1820: F, t1651: F, t597: F, t1828: F, t587: F, t1769: F, t562: F, t1821: F, t1630: F, t649: F) -> (F, F, F, F, F, F, F, F, F) {
    let t5126 = t5125 * t1823;
    let t5127 = t1820 * t5126;
    let t5128 = F::cast_from(32.0_f64) / F::cast_from(45.0_f64) * t5127;
    let t5129 = t1651 * t597;
    let t5130 = t5129 * t1828;
    let t5131 = t587 * t5130;
    let t5132 = F::cast_from(16.0_f64) / F::cast_from(45.0_f64) * t5131;
    let t5133 = t1769 * t562;
    let t5134 = t1821 * t5133;
    let t5136 = F::cast_from(8.0_f64) / F::cast_from(15.0_f64) * t1820 * t5134;
    let t5137 = t1630 * t649;
    (t5126, t5128, t5129, t5130, t5132, t5133, t5134, t5136, t5137)
}
