//! GGA_C_ACGGAP lxc pol — lxc_pol part 11 (v4rho3sigma_3) CSE chunk 1017/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part11_v4rho3sigma_3_chunk1017<F: Float>(t34023: F, t33983: F, t33984: F, t33987: F, t33990: F, t33995: F, t33997: F, t33998: F, t34000: F, t34003: F, t34005: F, t34009: F, t34011: F, t34014: F, t34015: F, t34017: F, t34019: F, t34021: F) -> F {
    let t34024 = F::cast_from(0.28303283060643736861e-1_f64) * t34023;
    let t34025 = -t33983 + F::cast_from(0.18868855373762491241e-2_f64) * t33984 + t33987 - F::cast_from(0.75475421495049964964e-2_f64) * t33990 + t33995 + t33997 + F::cast_from(0.34299214494455789578e-2_f64) * t33998 - F::cast_from(0.51448821741683684366e-2_f64) * t34000 + F::cast_from(0.42874018118069736972e-3_f64) * t34003 - F::cast_from(0.10289764348336736873e-1_f64) * t34005 + F::cast_from(0.21437009059034868486e-3_f64) * t34009 - F::cast_from(0.41930789719472202758e-2_f64) * t34011 + t34014 + F::cast_from(0.17149607247227894789e-2_f64) * t34015 + F::cast_from(0.85748036236139473944e-3_f64) * t34017 - F::cast_from(0.17149607247227894789e-2_f64) * t34019 - F::cast_from(0.42874018118069736972e-3_f64) * t34021 + t34024;
    t34025
}
