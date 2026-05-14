//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 910/1141 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk910<F: Float>(t12528: F, t636: F, t12870: F, t211: F, t582: F, t12563: F, t5480: F, t639: F, t12804: F, t17268: F, t587: F, t12809: F, t1820: F, t5125: F, t12588: F, t5175: F) -> (F, F, F, F, F, F) {
    let t41398 = t12528 * t636;
    let t41401 = t211 * t582 * t12870;
    let t41404 = t639 * t5480 * t12563;
    let t41418 = t587 * t17268 * t12804;
    let t41421 = t1820 * t5125 * t12809;
    let t41432 = t5175 * t12588;
    (t41398, t41401, t41404, t41418, t41421, t41432)
}
