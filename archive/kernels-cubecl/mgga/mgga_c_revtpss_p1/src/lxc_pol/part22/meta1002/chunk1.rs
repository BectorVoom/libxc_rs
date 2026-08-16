//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3410/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3410<F: Float>(t11452: F, t6173: F, t2986: F, t6184: F, t11399: F, t11404: F, t11450: F, t11507: F, t15263: F, t15267: F, t15290: F, t15339: F, t15340: F, t15350: F, t15400: F, t1622: F, t19227: F, t19275: F, t19279: F, t19282: F, t19283: F, t2938: F, t2944: F, t2962: F, t2968: F, t2988: F, t2989: F, t3006: F, t3012: F, t3014: F, t41662: F, t41775: F, t4647: F, t4670: F, t4673: F, t52642: F, t52830: F, t6158: F, t6174: F, t6190: F, t6209: F, t63902: F) -> F {
    let t63979 = t6173 * t11452;
    let t63997 = t6184 * t2986;
    let t64023 = F::cast_from(0.32163958997385070134e2_f64) * t2968 * t19275 * t2962 + F::cast_from(0.2069040516770936012e4_f64) * t11450 * t63979 * t2944 + F::cast_from(0.12865583598954028054e3_f64) * t11404 * t19279 + F::cast_from(0.64327917994770140268e2_f64) * t2968 * t4673 * t15339 + F::cast_from(0.4138081033541872024e4_f64) * t41662 * t19283 + F::cast_from(0.2069040516770936012e4_f64) * t11450 * t19282 * t2962 + F::cast_from(2.0_f64) * t52830 * t1622 + F::cast_from(4.0_f64) * t15400 * t4670 - F::cast_from(0.11696447245269292414e1_f64) * t63997 * t2989 + F::cast_from(2.0_f64) * t4647 * t15340 - F::cast_from(2.0_f64) * t41775 * t6158 + F::cast_from(1.0_f64) * t11399 * t6174 + F::cast_from(2.0_f64) * t2938 * t19227 + F::cast_from(0.34631718211362927518e2_f64) * t3012 * t63902 * t3014 + F::cast_from(0.70178683471615754484e1_f64) * t15350 * t15290 + F::cast_from(0.34631718211362927517e2_f64) * t15350 * t15263 + F::cast_from(0.20508037716432813315e4_f64) * t52642 * t15267 + F::cast_from(0.35089341735807877242e1_f64) * t3012 * t6190 * t3006 + F::cast_from(0.6233709278045326953e3_f64) * t11507 * t6209 * t2988;
    t64023
}
