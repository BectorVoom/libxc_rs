//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 705/1272 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk705<F: Float>(t2113: F, t2159: F, t3467: F, t673: F, t695: F, t6986: F, t6993: F, t6994: F, t6997: F, t7002: F, t7005: F, t7009: F, t7012: F, t7015: F, t7019: F, t7023: F, t7026: F, t7031: F) -> (F,) {
    let t7033 = -0.10431793787746509425e1 * t3467 * t6986 - 0.18137053605011111023e0 * t6993 * t6994 + 0.13602790203758333267e0 * t2159 * t6997 - 0.52158968938732547127e0 * t7002 * t7005 + 0.52158968938732547127e0 * t2113 * t7009 - 0.15114211337509259186e-1 * t695 * t7012 + 0.2115989587251296286e0 * t7015 - 0.6347968761753888858e0 * t7019 - 0.11990607661090678954e1 * t7023 - 0.86931614897887578546e-1 * t673 * t7026 - 0.20284043476173768327e1 * t7031;
    (t7033,)
}
