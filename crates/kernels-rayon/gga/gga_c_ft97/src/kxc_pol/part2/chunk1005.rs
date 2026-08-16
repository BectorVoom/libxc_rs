//! GGA_C_FT97 kxc pol — kxc_pol part 2 (v3rho3_1) CSE chunk 1005/1007 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_kxc_pol_part2_v3rho3_1_chunk1005(t15549: f64, t332: f64, t113: f64, t1259: f64, t14569: f64, t14571: f64, t14576: f64, t14579: f64, t14582: f64, t14593: f64, t1577: f64, t1934: f64, t2900: f64, t2958: f64, t2966: f64, t333: f64, t4318: f64, t4322: f64, t5: f64, t505: f64, t889: f64, t911: f64, t992: f64) -> f64 {
    let t15550 = t15549 * t332;
    let t15554 = -t889 * t14569 + t14571 * t911 / 2.0_f64 + t4322 * t2966 / 2.0_f64 + t889 * t14576 / 2.0_f64 + t889 * t14579 / 4.0_f64 + t889 * t14582 / 4.0_f64 + t5 * t4318 * t505 / 2.0_f64 + t5 * t2900 * t992 / 4.0_f64 + t4322 * t2958 / 4.0_f64 + 3.0_f64 / 2.0_f64 * t889 * t14593 - t5 * t333 * t1577 / 2.0_f64 + t5 * t1259 * t1934 / 4.0_f64 + t5 * t15550 * t113 / 4.0_f64;
    t15554
}
