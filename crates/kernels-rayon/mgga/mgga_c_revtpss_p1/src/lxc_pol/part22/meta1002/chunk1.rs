//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3410/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3410(t11452: f64, t6173: f64, t2986: f64, t6184: f64, t11399: f64, t11404: f64, t11450: f64, t11507: f64, t15263: f64, t15267: f64, t15290: f64, t15339: f64, t15340: f64, t15350: f64, t15400: f64, t1622: f64, t19227: f64, t19275: f64, t19279: f64, t19282: f64, t19283: f64, t2938: f64, t2944: f64, t2962: f64, t2968: f64, t2988: f64, t2989: f64, t3006: f64, t3012: f64, t3014: f64, t41662: f64, t41775: f64, t4647: f64, t4670: f64, t4673: f64, t52642: f64, t52830: f64, t6158: f64, t6174: f64, t6190: f64, t6209: f64, t63902: f64) -> f64 {
    let t63979 = t6173 * t11452;
    let t63997 = t6184 * t2986;
    let t64023 = 0.32163958997385070134e2_f64 * t2968 * t19275 * t2962 + 0.2069040516770936012e4_f64 * t11450 * t63979 * t2944 + 0.12865583598954028054e3_f64 * t11404 * t19279 + 0.64327917994770140268e2_f64 * t2968 * t4673 * t15339 + 0.4138081033541872024e4_f64 * t41662 * t19283 + 0.2069040516770936012e4_f64 * t11450 * t19282 * t2962 + 2.0_f64 * t52830 * t1622 + 4.0_f64 * t15400 * t4670 - 0.11696447245269292414e1_f64 * t63997 * t2989 + 2.0_f64 * t4647 * t15340 - 2.0_f64 * t41775 * t6158 + 1.0_f64 * t11399 * t6174 + 2.0_f64 * t2938 * t19227 + 0.34631718211362927518e2_f64 * t3012 * t63902 * t3014 + 0.70178683471615754484e1_f64 * t15350 * t15290 + 0.34631718211362927517e2_f64 * t15350 * t15263 + 0.20508037716432813315e4_f64 * t52642 * t15267 + 0.35089341735807877242e1_f64 * t3012 * t6190 * t3006 + 0.6233709278045326953e3_f64 * t11507 * t6209 * t2988;
    t64023
}
