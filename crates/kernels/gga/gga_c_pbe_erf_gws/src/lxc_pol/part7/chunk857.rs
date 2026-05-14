//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 857/1100 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk857<F: Float>(t17456: F, t17461: F, t17463: F, t17465: F, t17467: F, t17469: F, t17473: F, t17476: F, t17481: F, t17484: F, t17488: F, t1986: F, t1989: F, t1879: F, t5556: F, t1827: F, t418: F, t5273: F, t572: F, t587: F) -> (F, F, F, F) {
    let t17489 = -t17456 - t17461 + t17463 + t17465 + t17467 - t17469 + t17473 - t17476 - t17481 + t17484 - t17488;
    let t17490 = t1989 * t1986;
    let t17492 = t1879 * t5556;
    let t17493 = 32.0 / 45.0 * t17492;
    let t17498 = 16.0 / 45.0 * t587 * t1827 * t5273 * t572 * t418;
    (t17489, t17490, t17493, t17498)
}
