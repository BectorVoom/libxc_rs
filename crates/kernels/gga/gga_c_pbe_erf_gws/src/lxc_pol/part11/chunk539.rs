//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 539/1302 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk539<F: Float>(t127: F, t1511: F, t1519: F, t1540: F, t1555: F, t1561: F, t2879: F, t2891: F, t3648: F, t3651: F, t3654: F, t3657: F, t3661: F, t3665: F, t3668: F, t496: F) -> F {
    let t3671 = -t1511 + t3648 + t1519 + t3651 - t3654 + t1540 + t2879 / F::new(3.0) + F::new(3.0) / F::new(2.0) * t496 * t3657 - t496 * t3661 / F::new(2.0) + t1555 + F::new(0.146904e1) * t2891 + t1561 + F::new(0.587616e1) * t127 * t3665 - F::new(0.146904e1) * t127 * t3668;
    t3671
}
