//! GGA_C_ACGGAP lxc pol — lxc_pol part 13 (v4rho3sigma_5) CSE chunk 1017/1213 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part13_v4rho3sigma_5_chunk1017(t34023: f64, t33983: f64, t33984: f64, t33987: f64, t33990: f64, t33995: f64, t33997: f64, t33998: f64, t34000: f64, t34003: f64, t34005: f64, t34009: f64, t34011: f64, t34014: f64, t34015: f64, t34017: f64, t34019: f64, t34021: f64) -> f64 {
    let t34024 = 0.28303283060643736861e-1_f64 * t34023;
    let t34025 = -t33983 + 0.18868855373762491241e-2_f64 * t33984 + t33987 - 0.75475421495049964964e-2_f64 * t33990 + t33995 + t33997 + 0.34299214494455789578e-2_f64 * t33998 - 0.51448821741683684366e-2_f64 * t34000 + 0.42874018118069736972e-3_f64 * t34003 - 0.10289764348336736873e-1_f64 * t34005 + 0.21437009059034868486e-3_f64 * t34009 - 0.41930789719472202758e-2_f64 * t34011 + t34014 + 0.17149607247227894789e-2_f64 * t34015 + 0.85748036236139473944e-3_f64 * t34017 - 0.17149607247227894789e-2_f64 * t34019 - 0.42874018118069736972e-3_f64 * t34021 + t34024;
    t34025
}
