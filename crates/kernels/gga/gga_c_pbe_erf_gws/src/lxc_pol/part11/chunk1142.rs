//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1142/1302 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1142<F: Float>(t47450: F, t587: F, t7435: F, t12460: F, t1820: F, t995: F, t12805: F, t2615: F, t1017: F, t17260: F, t1022: F, t12513: F, t1620: F, t1809: F) -> (F, F, F, F, F) {
    let t48169 = F::cast_from(64.0_f64) / F::cast_from(27.0_f64) * t587 * t7435 * t47450;
    let t48173 = F::cast_from(256.0_f64) / F::cast_from(81.0_f64) * t1820 * t7435 * t12460 * t995;
    let t48175 = F::cast_from(16.0_f64) / F::cast_from(9.0_f64) * t2615 * t12805;
    let t48179 = F::cast_from(128.0_f64) / F::cast_from(81.0_f64) * t587 * t17260 * t12460 * t1017;
    let t48183 = F::cast_from(32.0_f64) / F::cast_from(45.0_f64) * t1620 * t1809 * t12513 * t1022;
    (t48169, t48173, t48175, t48179, t48183)
}
