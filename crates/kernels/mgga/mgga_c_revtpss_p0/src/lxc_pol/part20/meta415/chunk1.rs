//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1536/1798 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1536<F: Float>(t11630: F, t11633: F, t3172: F, t11988: F, t3106: F, t271: F, t2852: F, t41296: F, t1011: F, t1012: F, t1042: F, t1063: F, t11634: F, t11759: F, t12004: F, t3101: F, t3117: F, t3241: F, t3253: F, t39443: F, t39449: F, t42615: F, t43194: F, t43204: F, t43207: F, t4801: F, t4806: F, t4892: F, t4894: F) -> F {
    let t43211 = t11630 * t3172 * t11633;
    let t43215 = t3106 * t11988;
    let t43222 = F::new(1.0) / t271 / t2852;
    let t43223 = t43222 * t41296;
    let t43234 = -F::cast_from(0.11433071498151929859e-2_f64) * t1063 * t1042 * t4801 * t43194 + F::cast_from(0.95275595817932748828e-3_f64) * t1063 * t1042 * t4806 * t43194 + F::cast_from(0.38110238327173099531e-3_f64) * t43204 - F::cast_from(0.27439371595564631662e-1_f64) * t43207 * t11634 + F::cast_from(0.34299214494455789578e-2_f64) * t43211 - F::cast_from(0.57927562257303111285e-1_f64) * t12004 * t3101 + F::cast_from(0.20325460441158986416e-2_f64) * t43215 + F::cast_from(0.17149607247227894789e-2_f64) * t4892 * t3117 * t42615 * t4894 + F::new(35.0) / F::new(972.0) * t1011 * t1012 * t43223 * t39443 - t3241 * t11759 / F::new(27.0) + t1011 * t1012 * t3253 * t39449 / F::new(72.0);
    t43234
}
