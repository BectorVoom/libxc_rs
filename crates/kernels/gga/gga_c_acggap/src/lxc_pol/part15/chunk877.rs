//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 877/1124 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk877<F: Float>(t30601: F, t30605: F, t1061: F, t535: F, t7380: F, t1165: F, t33509: F, t604: F, t7346: F, t30613: F, t30468: F, t4425: F, t1470: F, t30644: F, t30984: F, t8458: F) -> (F, F, F, F, F, F, F, F, F) {
    let t34484 = t30601 / 64.0;
    let t34485 = t30605 / 192.0;
    let t34487 = t535 * t1061;
    let t34488 = t7380 * t34487;
    let t34492 = t7346 * t1165 * t604 * t33509;
    let t34499 = 0.25724410870841842184e-2 * t30613;
    let t34500 = t30468 * t4425;
    let t34506 = t30644 * t1470;
    let t34508 = t30984 * t8458;
    (t34484, t34485, t34487, t34488, t34492, t34499, t34500, t34506, t34508)
}
