//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1278/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1278(t1076: f64, t1118: f64, t1144: f64, t13112: f64, t13121: f64, t13141: f64, t13205: f64, t13212: f64, t13221: f64, t13607: f64, t13688: f64, t2408: f64, t2503: f64, t335: f64, t338: f64, t34850: f64, t34914: f64, t35003: f64, t353: f64, t35929: f64, t3916: f64, t43814: f64, t4386: f64, t44019: f64, t44021: f64, t6816: f64, t833: f64, t859: f64, t8787: f64, t9283: f64, t9815: f64) -> f64 {
    let t50479 = 7.0_f64 / 48.0_f64 * t34914 * t859 * t353 * t43814 * t1076 + t34850 * t13121 / 12.0_f64 + t34850 * t13112 / 6.0_f64 - t35003 * t4386 * t353 * t1118 * t1076 / 4.0_f64 - t335 * t338 * t1144 * t13607 / 24.0_f64 + 7.0_f64 / 12.0_f64 * t44019 + 7.0_f64 / 24.0_f64 * t44021 - t9815 * t13212 / 12.0_f64 - t2408 * t9283 * t8787 * t13205 / 2.0_f64 - 35.0_f64 / 36.0_f64 * t35929 + t3916 * t13141 * t833 / 32.0_f64 + t13688 * t2503 / 12.0_f64 - t6816 * t338 * t1144 * t13221;
    t50479
}
