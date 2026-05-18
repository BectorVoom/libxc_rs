//! GGA_C_ACGGAP lxc pol — lxc_pol part 13 (v4rho3sigma_5) CSE chunk 1095/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part13_v4rho3sigma_5_chunk1095<F: Float>(t1432: F, t1992: F, t30147: F, t7586: F, t30862: F, t30866: F, t30874: F, t30878: F, t30893: F, t30868: F, t30872: F, t30876: F, t30880: F, t30883: F, t30884: F, t30887: F, t30890: F, t30891: F, t30901: F, t30905: F, t30908: F) -> F {
    let t35022 = t30147 * t7586 * t1992 * t1432;
    let t35024 = F::new(0.12862205435420921092e-1) * t30862;
    let t35025 = F::new(0.17149607247227894789e-2) * t30866;
    let t35028 = F::new(0.32012600194825403606e-1) * t30874;
    let t35030 = F::new(0.16006300097412701803e-1) * t30878;
    let t35034 = F::new(0.28582678745379824648e-3) * t30893;
    let t35036 = F::new(0.28582678745379824648e-3) * t35022 - t35024 - t35025 + F::new(0.45351183609335988443e-1) * t30868 - F::new(0.45351183609335988443e-1) * t30872 + t35028 + F::new(0.80031500487063509016e-2) * t30876 - t35030 + F::new(0.90035438047946447642e-2) * t30880 + t30883 - F::new(0.40015750243531754508e-1) * t30884 - t30887 - t30890 + F::new(0.10718504529517434243e-3) * t30891 + t35034 + F::new(0.7145669686344956162e-4) * t30901 - t30905 - t30908;
    t35036
}
