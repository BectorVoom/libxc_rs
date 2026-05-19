//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 749/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk749<F: Float>(t184: F, t20989: F, t1078: F, t4893: F, t21: F, t1079: F, t4431: F, t4894: F, t920: F, t4889: F, t11167: F, t15734: F, t15750: F, t15760: F, t20025: F, t20029: F, t20033: F, t20037: F, t20041: F, t20047: F, t20589: F, t20599: F, t3359: F, t4466: F, t8698: F) -> (F, F, F, F, F, F, F) {
    let t20990 = t20989 * t184;
    let t20994 = t4893 * t1078;
    let t20995 = t20994 * t184;
    let t20996 = t20995 * t21;
    let t21002 = t1079 * t4431;
    let t21005 = t4894 * t920;
    let t21008 = t4889 * t920;
    let t21025 = F::new(0.1760655e0) * t20589 - F::new(0.352131e0) * t3359 * t4466 + F::new(0.234754e0) * t20599 - t8698 - F::cast_from(0.19257444444444444444e0_f64) * t11167 + F::cast_from(0.9628722222222222222e-1_f64) * t15734 - F::cast_from(0.28886166666666666666e0_f64) * t15750 + F::cast_from(0.14443083333333333333e0_f64) * t15760 - F::cast_from(0.1604787037037037037e0_f64) * t20025 + F::cast_from(0.57772333333333333332e0_f64) * t20029 - F::cast_from(0.28886166666666666666e0_f64) * t20033 - F::cast_from(0.86658499999999999998e0_f64) * t20037 + F::cast_from(0.86658499999999999998e0_f64) * t20041 - F::cast_from(0.14443083333333333333e0_f64) * t20047;
    (t20990, t20995, t20996, t21002, t21005, t21008, t21025)
}
