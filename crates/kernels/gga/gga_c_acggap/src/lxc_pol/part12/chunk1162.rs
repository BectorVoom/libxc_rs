//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 1162/1250 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk1162<F: Float>(t34027: F, t34031: F, t34033: F, t34035: F, t34037: F, t34039: F, t34043: F, t34052: F, t34056: F, t34068: F, t30260: F, t30265: F, t34029: F, t34041: F, t34048: F, t34054: F, t34059: F, t34063: F) -> F {
    let t36908 = F::cast_from(0.42874018118069736972e-2_f64) * t34027;
    let t36910 = F::cast_from(0.2264262644851498949e-1_f64) * t34031;
    let t36911 = F::cast_from(0.21437009059034868486e-3_f64) * t34033;
    let t36912 = F::cast_from(0.42874018118069736972e-3_f64) * t34035;
    let t36913 = F::cast_from(0.42874018118069736972e-3_f64) * t34037;
    let t36914 = F::cast_from(0.28582678745379824648e-3_f64) * t34039;
    let t36916 = F::cast_from(0.38110238327173099531e-2_f64) * t34043;
    let t36918 = F::cast_from(0.14291339372689912324e-2_f64) * t34052;
    let t36920 = F::cast_from(0.14291339372689912324e-2_f64) * t34056;
    let t36925 = F::cast_from(0.85748036236139473944e-3_f64) * t34068;
    let t36926 = t36908 + F::cast_from(0.25724410870841842184e-2_f64) * t34029 - t36910 - t36911 - t36912 - t36913 - t36914 - F::cast_from(0.17149607247227894789e-2_f64) * t34041 + t36916 - F::cast_from(0.10718504529517434243e-2_f64) * t34048 - t36918 - F::cast_from(0.26416397523267487738e-1_f64) * t34054 - t36920 - F::cast_from(0.27953859812981468505e-1_f64) * t30260 + F::cast_from(0.12579236915841660828e-2_f64) * t34059 - t34063 / F::new(192.0) - F::cast_from(0.83861579438944405516e-3_f64) * t30265 - t36925;
    t36926
}
