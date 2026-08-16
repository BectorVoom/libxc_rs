//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 993/1242 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk993<F: Float>(t1: F, t5560: F, t6006: F, t119: F, t1917: F, t5559: F, t17147: F, t17150: F, t17152: F, t17154: F, t17158: F, t17160: F, t17164: F, t17167: F, t17171: F) -> F {
    let t18220 = t6006 * t1 * t5560;
    let t18224 = F::cast_from(0.60617527037037037035e-2_f64) * t5559 * t119 * t1917;
    let t18225 = -t17147 + t17150 - t17152 - t17154 + t17158 - t17160 + F::cast_from(0.60617527037037037035e-2_f64) * t18220 + t18224 + t17164 + t17167 + t17171;
    t18225
}
