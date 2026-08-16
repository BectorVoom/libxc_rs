//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1278/1302 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1278<F: Float>(t1076: F, t1118: F, t1144: F, t13112: F, t13121: F, t13141: F, t13205: F, t13212: F, t13221: F, t13607: F, t13688: F, t2408: F, t2503: F, t335: F, t338: F, t34850: F, t34914: F, t35003: F, t353: F, t35929: F, t3916: F, t43814: F, t4386: F, t44019: F, t44021: F, t6816: F, t833: F, t859: F, t8787: F, t9283: F, t9815: F) -> F {
    let t50479 = F::cast_from(7.0_f64) / F::cast_from(48.0_f64) * t34914 * t859 * t353 * t43814 * t1076 + t34850 * t13121 / F::cast_from(12.0_f64) + t34850 * t13112 / F::cast_from(6.0_f64) - t35003 * t4386 * t353 * t1118 * t1076 / F::cast_from(4.0_f64) - t335 * t338 * t1144 * t13607 / F::cast_from(24.0_f64) + F::cast_from(7.0_f64) / F::cast_from(12.0_f64) * t44019 + F::cast_from(7.0_f64) / F::cast_from(24.0_f64) * t44021 - t9815 * t13212 / F::cast_from(12.0_f64) - t2408 * t9283 * t8787 * t13205 / F::cast_from(2.0_f64) - F::cast_from(35.0_f64) / F::cast_from(36.0_f64) * t35929 + t3916 * t13141 * t833 / F::cast_from(32.0_f64) + t13688 * t2503 / F::cast_from(12.0_f64) - t6816 * t338 * t1144 * t13221;
    t50479
}
