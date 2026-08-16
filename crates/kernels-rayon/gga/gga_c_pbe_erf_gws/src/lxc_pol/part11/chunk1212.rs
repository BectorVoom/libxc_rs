//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1212/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1212(t3721: f64, t1105: f64, t13086: f64, t1118: f64, t1162: f64, t12182: f64, t13112: f64, t13606: f64, t20173: f64, t2376: f64, t2408: f64, t2409: f64, t3207: f64, t335: f64, t338: f64, t34773: f64, t34922: f64, t353: f64, t35889: f64, t36323: f64, t3733: f64, t3742: f64, t3780: f64, t3917: f64, t39689: f64, t43788: f64, t43790: f64, t4386: f64, t47071: f64, t831: f64, t859: f64, t9820: f64) -> (f64, f64) {
    let t49172 = t3721 * t3721;
    let t49178 = t1105 * t13086;
    let t49192 = -t34773 * t4386 * t353 * t1118 * t3780 / 4.0_f64 + t36323 * t12182 / 4.0_f64 + t34922 * t13112 / 6.0_f64 + t39689 * t12182 / 4.0_f64 - t34773 * t859 * t353 * t1162 * t3780 / 8.0_f64 - t47071 * t3733 / 32.0_f64 + 3.0_f64 / 8.0_f64 * t3917 * t9820 - 7.0_f64 / 24.0_f64 * t43788 - 7.0_f64 / 24.0_f64 * t43790 + t335 * t338 * t353 * t20173 * t49172 / 4.0_f64 + t3207 * t2409 * t831 * t49178 / 4.0_f64 + t2408 * t2409 * t35889 * t3742 / 4.0_f64 + t2408 * t2409 * t2376 * t13606 * t1105 / 12.0_f64;
    (t49178, t49192)
}
