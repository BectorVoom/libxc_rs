//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 905/1042 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk905<F: Float>(t150704: F, t237: F, t15: F, t35437: F, t6793: F, t35435: F, t141166: F, t150594: F, t150630: F, t150687: F, t150688: F, t150694: F, t150697: F, t150699: F, t27547: F, t27696: F, t33351: F, t33388: F, t33436: F, t36796: F, t3734: F, t3752: F, t3786: F, t6046: F, t683: F) -> (F, F) {
    let t150705 = t150704 * t237;
    let t150709 = t6793 * t15 * t35437;
    let t150710 = t35435 * t150709;
    let t150722 = -0.15625977470667646633e-5 * t150687 * t150688 * t6046 + 0.22705522127871165896e-3 * t150694 + 0.26086440517961693841e-2 * t150697 - 0.35216694699248286684e-1 * t150699 * t33436 * t683 * t27547 - 0.46509801892875584e-1 * t150705 * t27696 + 0.28200083969358461043e-4 * t150710 - 0.22227677429409423704e-2 * t33388 * t150630 + 0.46509801892875584e-2 * t33351 * t3734 - 0.23254900946437792e-1 * t33351 * t3752 - 0.52700762016626893448e-4 * t36796 * t150594 + 0.23254900946437792e-1 * t141166 * t3786;
    (t150709, t150722)
}
