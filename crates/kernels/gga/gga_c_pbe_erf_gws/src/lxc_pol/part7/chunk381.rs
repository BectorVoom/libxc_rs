//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 381/1242 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk381<F: Float>(t1504: F, t1563: F, t127: F, t1511: F, t1517: F, t1519: F, t1522: F, t1533: F, t1536: F, t1540: F, t1542: F, t1545: F, t1549: F, t1555: F, t1558: F, t1561: F, t496: F, t506: F) -> (F, F) {
    let t1564 = t1563 * t1504;
    let t1570 = -t1511 + t1517 + t1519 + t1522 - t1536 + t1540 + t1542 / F::new(3.0) + F::new(3.0) / F::new(2.0) * t496 * t1545 - t496 * t1549 / F::new(2.0) + t1555 + F::new(0.146904e1) * t1558 + t1561 + F::new(0.587616e1) * t127 * t1564 - F::new(0.146904e1) * t127 * t506 * t1533;
    (t1564, t1570)
}
