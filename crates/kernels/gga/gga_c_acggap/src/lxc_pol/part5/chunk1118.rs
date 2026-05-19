//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1118/1332 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1118<F: Float>(t15016: F, t15018: F, t11841: F, t11843: F, t11849: F, t6021: F, t912: F, t1: F, t283: F, t5474: F, t1708: F, t40: F, t803: F) -> (F, F, F, F, F, F, F, F) {
    let t20007 = F::new(160.0) * t15016;
    let t20008 = F::new(240.0) * t15018;
    let t20009 = F::new(24.0) * t11841;
    let t20010 = F::new(240.0) * t11843;
    let t20011 = F::new(2.0) * t11849;
    let t20012 = t6021 * t912;
    let t20013 = F::cast_from(0.11696447245269292414e1_f64) * t20012;
    let t20015 = t5474 * t1 * t283;
    let t20016 = F::cast_from(0.36622894612013090108e-3_f64) * t20015;
    let t20018 = t40 * t1708 * t803;
    (t20007, t20008, t20009, t20010, t20011, t20013, t20016, t20018)
}
