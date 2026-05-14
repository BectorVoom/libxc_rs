//! GGA_C_FT97 kxc pol — kxc_pol part 2 (v3rho3_1) CSE chunk 366/869 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part2_v3rho3_1_chunk366<F: Float>(t2071: F, t550: F, t133: F, t1355: F, t140: F, t1683: F, t1698: F, t1993: F, t1996: F, t2001: F, t2003: F, t2032: F, t2036: F, t2038: F, t2043: F, t2045: F, t2060: F, t399: F, t540: F, t543: F) -> (F,) {
    let t2072 = t550 * t2071;
    let t2074 = 2.0 * t1993 - 0.2416365355361531912e1 * t1996 * t399 + 0.2416365355361531912e1 * t540 * t399 - 4.0 * t2001 * t2003 + 2.0 * t2032 + 0.72985269132393279984e0 * t2036 * t2038 - 0.29194107652957311994e1 * t543 * t1698 + 0.1208182677680765956e1 * t2043 * t2045 + 0.38259118126557588605e1 * t543 * t1683 + 0.14597053826478655997e1 * t140 * t1698 - 0.1208182677680765956e1 * t1355 * t2045 - 0.38259118126557588605e1 * t140 * t1683 + 2.0 * t133 * t2060 - t133 * t2072;
    (t2074,)
}
