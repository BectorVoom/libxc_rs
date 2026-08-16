//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 1386/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk1386(t15121: f64, t804: f64, t15389: f64, t321: f64, t14831: f64, t30104: f64, t12275: f64, t14825: f64, t15128: f64, t11889: f64, t13756: f64, t14149: f64, t14383: f64, t14821: f64, t15124: f64, t3928: f64, t3944: f64, t3946: f64, t4062: f64, t52774: f64, t52823: f64, t52853: f64, t52855: f64, t52860: f64) -> f64 {
    let t57799 = t804 * t15121;
    let t57801 = t321 * t15389;
    let t57803 = t30104 * t14831;
    let t57809 = t12275 * t14825;
    let t57817 = t804 * t15128;
    let t57819 = 12.0_f64 * t11889 * t13756 * t3944 - 6.0_f64 * t14149 * t15124 * t3946 - t14149 * t3928 * t4062 - 6.0_f64 * t14383 * t14821 * t3946 - 6.0_f64 * t14821 * t14825 * t3946 + 12.0_f64 * t52774 * t57803 - 12.0_f64 * t52823 * t57809 + t52853 + t52855 - t52860 + 6.0_f64 * t57799 + 2.0_f64 * t57801 + 3.0_f64 * t57817;
    t57819
}
