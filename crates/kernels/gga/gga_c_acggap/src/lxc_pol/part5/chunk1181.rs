//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1181/1332 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1181<F: Float>(t13263: F, t1750: F, t3379: F, t6255: F, t5891: F, t1165: F, t1180: F, t13100: F, t13110: F, t13112: F, t1531: F, t1532: F, t16602: F, t16608: F, t16610: F, t335: F, t336: F, t5080: F, t5630: F, t5852: F, t839: F, t945: F) -> F {
    let t21455 = t13263 * t1750;
    let t21457 = t3379 * t6255;
    let t21464 = t3379 * t5891;
    let t21467 = -t335 * t336 * t5630 * t839 / F::cast_from(48.0_f64) - F::cast_from(0.56688979511669985553e-2_f64) * t13100 + F::cast_from(0.85748036236139473945e-2_f64) * t13110 + F::cast_from(0.40015750243531754508e-1_f64) * t13112 - F::cast_from(0.42874018118069736972e-3_f64) * t1180 * t1165 * t1532 * t5080 - F::cast_from(0.80031500487063509016e-2_f64) * t16602 + F::cast_from(0.17149607247227894789e-2_f64) * t21455 + F::cast_from(0.34299214494455789578e-2_f64) * t21457 - F::cast_from(0.32012600194825403606e-1_f64) * t16608 + F::cast_from(0.30011812682648815881e-2_f64) * t1531 * t1165 * t5852 * t945 + F::cast_from(0.17149607247227894789e-2_f64) * t21464 - F::cast_from(0.80031500487063509016e-2_f64) * t16610;
    t21467
}
