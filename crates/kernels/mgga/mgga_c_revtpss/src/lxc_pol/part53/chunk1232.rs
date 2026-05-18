//! MGGA_C_REVTPSS lxc pol — lxc_pol part 53 (v4rho2sigma2_8) CSE chunk 1232/1244 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part53_v4rho2sigma2_8_chunk1232<F: Float>(t29427: F, t7002: F, t122950: F, t125209: F, t129270: F, t129431: F, t129478: F, t129479: F, t129480: F, t129481: F, t129482: F, t129483: F, t129488: F, t129489: F, t1518: F, t32175: F, t32825: F, t33643: F, t33645: F, t4292: F, t670: F, t8563: F) -> F {
    let t129490 = t29427 * t7002;
    let t129491 = t122950 * t1518 + t129270 * t670 + t129431 * t1518 + t32825 * t4292 + t125209 + t129478 + t129479 + t129480 + t129481 + t129482 + t129483 + t129488 + t129489 + t129490 + t32175 + t33643 + t33645 + t8563;
    t129491
}
