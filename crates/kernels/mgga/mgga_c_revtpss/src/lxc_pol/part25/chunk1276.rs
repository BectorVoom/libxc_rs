//! MGGA_C_REVTPSS lxc pol — lxc_pol part 25 (v4rho3sigma_0) CSE chunk 1276/1360 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part25_v4rho3sigma_0_chunk1276<F: Float>(t25577: F, t3111: F, t1020: F, t25576: F, t1047: F, t1068: F, t11653: F, t11689: F, t11693: F, t11707: F, t11862: F, t11871: F, t11930: F, t25517: F, t25526: F, t25580: F, t27493: F, t27498: F, t27536: F, t3120: F, t3130: F, t3136: F, t3164: F, t93646: F, t93649: F, t93655: F, t93658: F, t93667: F, t93670: F) -> F {
    let t93673 = t25577 * t3111;
    let t93675 = t1020 * t25576;
    let t93678 = F::new(0.14291339372689912324e-2) * t25517 * t11707 + F::new(0.17149607247227894789e-2) * t27536 * t11653 + F::new(0.91464571985215438873e-2) * t93646 * t3130 - F::new(0.13719685797782315831e-1) * t93649 * t1047 - F::new(0.68598428988911579154e-2) * t25526 * t3136 + F::new(0.68598428988911579154e-2) * t93655 * t3164 - F::new(0.25724410870841842183e-2) * t93658 * t11862 + F::new(0.25724410870841842183e-2) * t27493 * t11689 - F::new(0.12862205435420921092e-2) * t27498 * t11693 - F::new(0.12862205435420921092e-2) * t25580 * t11871 + F::new(0.25724410870841842183e-2) * t93667 * t11930 + F::new(0.13719685797782315831e-1) * t93670 * t3120 - F::new(0.60976381323476959248e-2) * t93673 - F::new(0.91464571985215438873e-2) * t93675 * t1068;
    t93678
}
