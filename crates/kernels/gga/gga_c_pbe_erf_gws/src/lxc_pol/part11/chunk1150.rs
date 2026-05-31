//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1150/1302 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1150<F: Float>(t12779: F, t2615: F, t42014: F, t42037: F, t33149: F, t33152: F, t42050: F, t25349: F, t48291: F, t48295: F, t48299: F, t48303: F) -> (F, F, F, F, F, F, F) {
    let t48305 = F::cast_from(32.0_f64) / F::cast_from(15.0_f64) * t2615 * t12779;
    let t48306 = F::cast_from(32.0_f64) / F::cast_from(15.0_f64) * t42014;
    let t48307 = F::cast_from(32.0_f64) / F::cast_from(45.0_f64) * t42037;
    let t48309 = F::cast_from(64.0_f64) / F::cast_from(135.0_f64) * t33149;
    let t48310 = F::cast_from(32.0_f64) / F::cast_from(135.0_f64) * t33152;
    let t48311 = F::cast_from(32.0_f64) / F::cast_from(45.0_f64) * t42050;
    let t48312 = -t48291 - t48295 + t48299 + t48303 + t48305 + t48306 + t48307 + F::cast_from(32.0_f64) / F::cast_from(81.0_f64) * t25349 - t48309 + t48310 - t48311;
    (t48305, t48306, t48307, t48309, t48310, t48311, t48312)
}
