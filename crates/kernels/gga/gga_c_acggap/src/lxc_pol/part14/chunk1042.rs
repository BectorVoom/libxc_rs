//! GGA_C_ACGGAP lxc pol — lxc_pol part 14 (v4rho3sigma_6) CSE chunk 1042/1223 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part14_v4rho3sigma_6_chunk1042<F: Float>(t34009: F, t34033: F, t34039: F, t34043: F, t34056: F, t34068: F, t34127: F, t34156: F, t34179: F, t34237: F, t34255: F, t34271: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t36898 = F::new(0.42874018118069736972e-3) * t34009;
    let t36911 = F::new(0.21437009059034868486e-3) * t34033;
    let t36914 = F::new(0.28582678745379824648e-3) * t34039;
    let t36916 = F::new(0.38110238327173099531e-2) * t34043;
    let t36920 = F::new(0.14291339372689912324e-2) * t34056;
    let t36925 = F::new(0.85748036236139473944e-3) * t34068;
    let t36950 = F::new(0.28582678745379824648e-3) * t34127;
    let t36961 = F::new(0.18868855373762491241e-2) * t34156;
    let t36970 = F::new(0.20965394859736101378e-2) * t34179;
    let t36993 = F::new(0.42874018118069736972e-3) * t34237;
    let t36998 = F::new(0.85748036236139473944e-3) * t34255;
    let t37003 = F::new(0.17149607247227894789e-2) * t34271;
    (t36898, t36911, t36914, t36916, t36920, t36925, t36950, t36961, t36970, t36993, t36998, t37003)
}
