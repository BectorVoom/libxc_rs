//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 950/1302 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk950<F: Float>(t1076: F, t153: F, t4573: F, t1072: F, t168: F, t5589: F, t3013: F, t700: F, t1061: F, t256: F, t5426: F, t2654: F, t5421: F) -> (F, F, F, F, F) {
    let t22766 = t153 * t4573 * t1076;
    let t22778 = t168 * t5589 * t1072;
    let t22800 = t3013 * t700;
    let t22811 = t1061 * t5426 * t256;
    let t22813 = t2654 * t5421;
    (t22766, t22778, t22800, t22811, t22813)
}
