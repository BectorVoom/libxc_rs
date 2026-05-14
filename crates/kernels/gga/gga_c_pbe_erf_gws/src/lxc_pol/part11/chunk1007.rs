//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1007/1141 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1007<F: Float>(t12805: F, t2615: F, t1017: F, t12460: F, t17260: F, t587: F, t1022: F, t12513: F, t1620: F, t1809: F, t3473: F, t3562: F, t1044: F, t1815: F, t639: F, t16801: F, t42094: F, t954: F) -> (F, F, F, F, F, F) {
    let t48175 = 16.0 / 9.0 * t2615 * t12805;
    let t48179 = 128.0 / 81.0 * t587 * t17260 * t12460 * t1017;
    let t48183 = 32.0 / 45.0 * t1620 * t1809 * t12513 * t1022;
    let t48187 = 16.0 / 15.0 * t1620 * t1809 * t3473 * t3562;
    let t48191 = 16.0 / 45.0 * t639 * t1815 * t12513 * t1044;
    let t48195 = 32.0 / 15.0 * t639 * t16801 * t42094 * t954;
    (t48175, t48179, t48183, t48187, t48191, t48195)
}
