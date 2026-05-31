//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 3115/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3115<F: Float>(t58452: F, t68454: F, t68456: F, t68538: F, t68540: F, t68548: F, t68550: F, t68567: F, t68583: F, t68585: F, t68590: F, t81539: F) -> F {
    let t81995 = F::cast_from(0.5519e-1_f64) * t81539 - F::cast_from(0.66228e0_f64) * t68538 - F::cast_from(0.99342e0_f64) * t68540 + F::cast_from(0.11038e0_f64) * t68548 + F::cast_from(0.33114e0_f64) * t68550 - F::cast_from(0.12077e1_f64) * t68454 - F::cast_from(0.181155e1_f64) * t68456 - F::cast_from(0.16557e0_f64) * t68567 + t58452 + F::cast_from(0.27595e0_f64) * t68583 + F::cast_from(0.5519e0_f64) * t68585 - F::cast_from(0.91983333333333333334e-1_f64) * t68590;
    t81995
}
