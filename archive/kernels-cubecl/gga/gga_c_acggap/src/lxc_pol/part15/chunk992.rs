//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 992/1278 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk992<F: Float>(t30798: F, t30830: F, t30854: F, t1432: F, t1992: F, t30147: F, t7586: F, t30862: F, t30866: F, t30874: F, t30878: F, t30893: F) -> (F, F, F, F, F, F, F, F, F) {
    let t35004 = F::cast_from(0.21437009059034868486e-3_f64) * t30798;
    let t35012 = F::cast_from(0.20965394859736101379e-2_f64) * t30830;
    let t35018 = F::cast_from(0.25724410870841842184e-2_f64) * t30854;
    let t35022 = t30147 * t7586 * t1992 * t1432;
    let t35024 = F::cast_from(0.12862205435420921092e-1_f64) * t30862;
    let t35025 = F::cast_from(0.17149607247227894789e-2_f64) * t30866;
    let t35028 = F::cast_from(0.32012600194825403606e-1_f64) * t30874;
    let t35030 = F::cast_from(0.16006300097412701803e-1_f64) * t30878;
    let t35034 = F::cast_from(0.28582678745379824648e-3_f64) * t30893;
    (t35004, t35012, t35018, t35022, t35024, t35025, t35028, t35030, t35034)
}
