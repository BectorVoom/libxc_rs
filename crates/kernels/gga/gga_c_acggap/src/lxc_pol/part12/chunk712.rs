//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 712/1092 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk712<F: Float>(t7747: F, t7775: F, t7781: F, t7787: F, t7800: F, t7802: F, t7749: F, t7751: F, t7754: F, t7756: F, t7758: F, t7760: F, t7764: F, t7768: F, t7771: F, t7773: F, t7785: F, t7789: F, t7793: F, t7797: F) -> (F, F, F, F, F, F, F) {
    let t8257 = 0.80031500487063509014e-2 * t7747;
    let t8268 = 0.19055119163586549766e-2 * t7775;
    let t8269 = 0.90035438047946447644e-2 * t7781;
    let t8271 = 0.13208198761633743869e-1 * t7787;
    let t8275 = 0.28582678745379824648e-3 * t7800;
    let t8276 = 0.31448092289604152069e-3 * t7802;
    let t8277 = t8257 + 0.51448821741683684367e-2 * t7749 + 0.17149607247227894789e-1 * t7751 - 0.57165357490759649296e-3 * t7754 + 0.12862205435420921092e-1 * t7756 + 0.51448821741683684367e-2 * t7758 - 0.21437009059034868486e-2 * t7760 - 0.21437009059034868486e-2 * t7764 - 0.10718504529517434243e-2 * t7768 - 0.14291339372689912324e-2 * t7771 - 0.25724410870841842184e-2 * t7773 - t8268 + t8269 - 0.62896184579208304138e-3 * t7785 - t8271 + 0.42874018118069736972e-3 * t7789 + 0.21437009059034868486e-3 * t7793 + 0.28582678745379824648e-3 * t7797 + t8275 - t8276;
    (t8257, t8268, t8269, t8271, t8275, t8276, t8277)
}
