//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 733/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk733(t678: f64, t7030: f64, t2113: f64, t2159: f64, t3467: f64, t673: f64, t695: f64, t6986: f64, t6993: f64, t6994: f64, t6997: f64, t7002: f64, t7005: f64, t7009: f64, t7012: f64, t7015: f64, t7019: f64, t7023: f64, t7026: f64) -> f64 {
    let t7031 = t7030 * t678;
    let t7033 = -0.10431793787746509425e1_f64 * t3467 * t6986 - 0.18137053605011111023e0_f64 * t6993 * t6994 + 0.13602790203758333267e0_f64 * t2159 * t6997 - 0.52158968938732547127e0_f64 * t7002 * t7005 + 0.52158968938732547127e0_f64 * t2113 * t7009 - 0.15114211337509259186e-1_f64 * t695 * t7012 + 0.2115989587251296286e0_f64 * t7015 - 0.6347968761753888858e0_f64 * t7019 - 0.11990607661090678954e1_f64 * t7023 - 0.86931614897887578546e-1_f64 * t673 * t7026 - 0.20284043476173768327e1_f64 * t7031;
    t7033
}
