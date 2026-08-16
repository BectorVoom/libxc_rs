//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1018/1302 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1018<F: Float>(t12747: F, t1620: F, t4934: F, t12599: F, t16666: F, t12528: F, t636: F, t12870: F, t211: F, t582: F, t12563: F, t5480: F, t639: F) -> (F, F, F, F, F) {
    let t41388 = t1620 * t4934 * t12747;
    let t41395 = t16666 * t12599;
    let t41398 = t12528 * t636;
    let t41401 = t211 * t582 * t12870;
    let t41404 = t639 * t5480 * t12563;
    (t41388, t41395, t41398, t41401, t41404)
}
