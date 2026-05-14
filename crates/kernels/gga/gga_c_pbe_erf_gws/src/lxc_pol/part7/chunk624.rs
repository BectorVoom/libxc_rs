//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 624/1100 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk624<F: Float>(t205: F, t626: F, t191: F, t1641: F, t261: F, t4367: F, t1691: F, t5024: F, t11: F, t5029: F, t5033: F, t625: F, t5038: F, t174: F, t838: F, t1243: F, t628: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t5060 = 1.0 / t205 / t626;
    let t5061 = t191 * t5060;
    let t5063 = 1.0 / t1641 / t261;
    let t5064 = t5063 * t4367;
    let t5065 = t5061 * t5064;
    let t5068 = t1691 * t5024;
    let t5069 = t11 * t5068;
    let t5071 = t1691 * t5029;
    let t5072 = t11 * t5071;
    let t5074 = t625 * t5033;
    let t5075 = t11 * t5074;
    let t5077 = t625 * t5038;
    let t5078 = t11 * t5077;
    let t5081 = t174 * t838 * t205;
    let t5082 = 0.11197407407407407407e0 * t5081;
    let t5083 = t1243 * t628;
    (t5061, t5063, t5064, t5065, t5068, t5069, t5071, t5072, t5074, t5075, t5077, t5078, t5081, t5082, t5083)
}
