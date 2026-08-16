//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1187/1242 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk1187<F: Float>(t274: F, t322: F, t6094: F, t2108: F, t1452: F, t18987: F, t20992: F, t2102: F, t2107: F, t21074: F, t21077: F, t21082: F, t323: F, t4867: F, t6086: F, t6089: F, t6096: F, t6097: F, t6100: F, t745: F, t818: F) -> F {
    let t21091 = t322 / t6094 / t274;
    let t21092 = t2108 * t2108;
    let t21098 = t1452 * t1452;
    let t21105 = -F::cast_from(36.0_f64) * t1452 * t2108 * t6096 + F::cast_from(8.0_f64) * t2107 * t4867 * t745 - F::cast_from(6.0_f64) * t1452 * t6086 - t18987 * t818 + t20992 * t323 - F::cast_from(4.0_f64) * t2102 * t4867 + F::cast_from(6.0_f64) * t2107 * t21098 - F::cast_from(4.0_f64) * t21074 * t745 + F::cast_from(12.0_f64) * t21077 * t2108 - F::cast_from(24.0_f64) * t21082 * t6097 + F::cast_from(24.0_f64) * t21091 * t21092 + F::cast_from(24.0_f64) * t6089 * t6100;
    t21105
}
