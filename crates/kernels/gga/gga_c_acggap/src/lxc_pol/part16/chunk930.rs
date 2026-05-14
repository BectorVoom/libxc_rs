//! GGA_C_ACGGAP lxc pol — lxc_pol part 16 (v4rho3sigma_8) CSE chunk 930/1080 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part16_v4rho3sigma_8_chunk930<F: Float>(t33867: F, t33869: F, t33874: F, t33894: F, t33960: F, t33984: F, t34009: F, t34033: F, t34039: F, t34043: F, t34056: F, t34068: F, t34127: F, t34156: F, t34179: F, t34237: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t36828 = 0.85748036236139473944e-3 * t33867;
    let t36829 = 0.15724046144802076034e-2 * t33869;
    let t36833 = 0.10718504529517434243e-2 * t33874;
    let t36838 = 0.28582678745379824648e-3 * t33894;
    let t36876 = 0.7640625e-2 * t33960;
    let t36889 = 0.37737710747524982482e-2 * t33984;
    let t36898 = 0.42874018118069736972e-3 * t34009;
    let t36911 = 0.21437009059034868486e-3 * t34033;
    let t36914 = 0.28582678745379824648e-3 * t34039;
    let t36916 = 0.38110238327173099531e-2 * t34043;
    let t36920 = 0.14291339372689912324e-2 * t34056;
    let t36925 = 0.85748036236139473944e-3 * t34068;
    let t36950 = 0.28582678745379824648e-3 * t34127;
    let t36961 = 0.18868855373762491241e-2 * t34156;
    let t36970 = 0.20965394859736101378e-2 * t34179;
    let t36993 = 0.42874018118069736972e-3 * t34237;
    (t36828, t36829, t36833, t36838, t36876, t36889, t36898, t36911, t36914, t36916, t36920, t36925, t36950, t36961, t36970, t36993)
}
