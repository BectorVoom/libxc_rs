//! GGA_C_ACGGAP lxc pol — lxc_pol part 13 (v4rho3sigma_5) CSE chunk 429/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part13_v4rho3sigma_5_chunk429<F: Float>(t145: F, t154: F, t355: F, t2035: F, t1969: F, t1971: F, t1974: F, t1987: F, t1990: F, t1996: F, t2000: F, t2002: F, t2004: F, t2006: F, t2011: F, t2014: F, t2018: F, t2022: F, t2024: F, t2026: F, t2033: F) -> (F, F, F) {
    let t2037 = t154 * t355 * t145;
    let t2038 = t2035 * t2037;
    let t2039 = t2038 / F::cast_from(192.0_f64);
    let t2040 = t1969 - t1971 + F::cast_from(0.10718504529517434243e-3_f64) * t1974 + t1987 - t1990 - t1996 - t2000 - F::cast_from(0.17149607247227894789e-2_f64) * t2002 + F::cast_from(0.85748036236139473944e-3_f64) * t2004 - F::cast_from(0.85748036236139473944e-3_f64) * t2006 - t2011 + t2014 + t2018 - t2022 - t2024 + t2026 / F::cast_from(96.0_f64) + F::cast_from(0.114609375e-1_f64) * t2033 + t2039;
    (t2037, t2039, t2040)
}
