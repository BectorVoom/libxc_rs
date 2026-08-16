//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2968/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2968<F: Float>(t77647: F, t77657: F, t78417: F, t78422: F, t78426: F, t78428: F, t78432: F, t78435: F, t78438: F, t78441: F, t78443: F, t78446: F, t78449: F, t78451: F, t78456: F, t78458: F, t78460: F, t78463: F, t78465: F, t78469: F) -> F {
    let t78683 = t77647 - t78417 + t78422 - t78426 - t78428 - t78432 + t78435 - t78438 - t78441 - t78443 + t77657 - t78446 + t78449 + t78451 - t78456 + t78458 + t78460 + t78463 - t78465 + t78469;
    t78683
}
