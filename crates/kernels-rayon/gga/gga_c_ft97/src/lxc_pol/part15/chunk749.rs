//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 749/1222 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk749(t184: f64, t20989: f64, t1078: f64, t4893: f64, t21: f64, t1079: f64, t4431: f64, t4894: f64, t920: f64, t4889: f64, t11167: f64, t15734: f64, t15750: f64, t15760: f64, t20025: f64, t20029: f64, t20033: f64, t20037: f64, t20041: f64, t20047: f64, t20589: f64, t20599: f64, t3359: f64, t4466: f64, t8698: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t20990 = t20989 * t184;
    let t20994 = t4893 * t1078;
    let t20995 = t20994 * t184;
    let t20996 = t20995 * t21;
    let t21002 = t1079 * t4431;
    let t21005 = t4894 * t920;
    let t21008 = t4889 * t920;
    let t21025 = 0.1760655e0_f64 * t20589 - 0.352131e0_f64 * t3359 * t4466 + 0.234754e0_f64 * t20599 - t8698 - 0.19257444444444444444e0_f64 * t11167 + 0.9628722222222222222e-1_f64 * t15734 - 0.28886166666666666666e0_f64 * t15750 + 0.14443083333333333333e0_f64 * t15760 - 0.1604787037037037037e0_f64 * t20025 + 0.57772333333333333332e0_f64 * t20029 - 0.28886166666666666666e0_f64 * t20033 - 0.86658499999999999998e0_f64 * t20037 + 0.86658499999999999998e0_f64 * t20041 - 0.14443083333333333333e0_f64 * t20047;
    (t20990, t20995, t20996, t21002, t21005, t21008, t21025)
}
