//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1456/1798 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1456<F: Float>(t291: F, t41554: F, t41567: F, t11545: F, t914: F, t936: F, t41481: F, t41483: F, t41485: F, t41488: F, t41490: F, t41493: F, t41496: F, t41505: F, t41509: F, t41513: F, t41542: F) -> (F, F, F) {
    let t41570 = F::cast_from(0.621814e-1_f64) * (t41554 + t41567) * t291;
    let t41571 = t11545 * t914;
    let t41573 = F::cast_from(4.0_f64) * t41571 * t936;
    let t41574 = -t41481 - t41483 - t41485 - t41488 + t41490 - t41493 + t41496 + t41505 - t41509 + t41513 + t41542 - t41570 + t41573;
    (t41570, t41573, t41574)
}
