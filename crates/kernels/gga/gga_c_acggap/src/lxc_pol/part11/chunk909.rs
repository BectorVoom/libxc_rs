//! GGA_C_ACGGAP lxc pol — lxc_pol part 11 (v4rho3sigma_3) CSE chunk 909/1066 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part11_v4rho3sigma_3_chunk909<F: Float>(t4439: F, t7822: F, t4681: F, t4443: F, t30543: F, t8661: F, t33983: F, t33984: F, t33987: F, t33990: F, t33995: F, t33997: F, t33998: F, t34000: F, t34003: F, t34005: F, t34009: F, t34011: F, t34014: F, t34015: F) -> (F,) {
    let t34017 = t7822 * t4439;
    let t34019 = t7822 * t4681;
    let t34021 = t7822 * t4443;
    let t34023 = t30543 * t8661;
    let t34024 = 0.28303283060643736861e-1 * t34023;
    let t34025 = -t33983 + 0.18868855373762491241e-2 * t33984 + t33987 - 0.75475421495049964964e-2 * t33990 + t33995 + t33997 + 0.34299214494455789578e-2 * t33998 - 0.51448821741683684366e-2 * t34000 + 0.42874018118069736972e-3 * t34003 - 0.10289764348336736873e-1 * t34005 + 0.21437009059034868486e-3 * t34009 - 0.41930789719472202758e-2 * t34011 + t34014 + 0.17149607247227894789e-2 * t34015 + 0.85748036236139473944e-3 * t34017 - 0.17149607247227894789e-2 * t34019 - 0.42874018118069736972e-3 * t34021 + t34024;
    (t34025,)
}
