//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 1032/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk1032(t33433: f64, t3766: f64, t1410: f64, t202: f64, t237: f64, t15: f64, t35437: f64, t6793: f64, t35435: f64, t141166: f64, t150594: f64, t150630: f64, t150687: f64, t150688: f64, t150694: f64, t150697: f64, t27547: f64, t27696: f64, t33351: f64, t33388: f64, t33436: f64, t36796: f64, t3734: f64, t3752: f64, t3786: f64, t6046: f64, t683: f64) -> (f64, f64) {
    let t150699 = t3766 * t33433;
    let t150704 = t202 * t1410;
    let t150705 = t150704 * t237;
    let t150709 = t6793 * t15 * t35437;
    let t150710 = t35435 * t150709;
    let t150722 = -0.15625977470667646633e-5_f64 * t150687 * t150688 * t6046 + 0.22705522127871165896e-3_f64 * t150694 + 0.26086440517961693841e-2_f64 * t150697 - 0.35216694699248286684e-1_f64 * t150699 * t33436 * t683 * t27547 - 0.46509801892875584e-1_f64 * t150705 * t27696 + 0.28200083969358461043e-4_f64 * t150710 - 0.22227677429409423704e-2_f64 * t33388 * t150630 + 0.46509801892875584e-2_f64 * t33351 * t3734 - 0.23254900946437792e-1_f64 * t33351 * t3752 - 0.52700762016626893448e-4_f64 * t36796 * t150594 + 0.23254900946437792e-1_f64 * t141166 * t3786;
    (t150709, t150722)
}
