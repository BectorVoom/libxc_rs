//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 771/1302 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk771<F: Float>(t12350: F, t1643: F, t642: F, t639: F, t12530: F, t12534: F, t12536: F, t12540: F, t12542: F, t12546: F, t12548: F, t12552: F, t12554: F, t12558: F, t12562: F, t12566: F, t12568: F, t12569: F, t12570: F, t12574: F) -> (F, F, F, F) {
    let t12575 = t1643 * t12350;
    let t12576 = t642 * t12575;
    let t12578 = F::cast_from(8.0_f64) / F::cast_from(15.0_f64) * t639 * t12576;
    let t12579 = t12530 + t12534 - t12536 - t12540 + t12542 - t12546 + t12548 + t12552 + t12554 + t12558 + t12562 - t12566 + t12568 + t12569 + t12570 + t12574 + t12578;
    (t12575, t12576, t12578, t12579)
}
