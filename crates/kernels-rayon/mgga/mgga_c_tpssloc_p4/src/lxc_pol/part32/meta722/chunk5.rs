//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 2304/2369 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2304(t1209: f64, t1751: f64, t17686: f64, t24589: f64, t24812: f64, t24813: f64, t27490: f64, t27491: f64, t27496: f64, t27497: f64, t27501: f64, t27536: f64, t27550: f64, t27644: f64, t29734: f64, t3247: f64, t3502: f64, t3961: f64, t5012: f64, t7373: f64, t86037: f64, t94796: f64, t94797: f64, t94847: f64, t94881: f64, t94885: f64, t94889: f64, t94891: f64, t94901: f64, t94954: f64, t94963: f64) -> f64 {
    let t103659 = -0.16449340668482264365e-1_f64 * t24812 * t27496 * t27490 * t5012 + 0.3289868133696452873e-1_f64 * t24812 * t24813 * t3502 * t1751 * t27491 - 0.16449340668482264365e-1_f64 * t24812 * t24813 * t1209 * t1751 * t27497 - 0.16449340668482264365e-1_f64 * t7373 * t27536 * t27501 + 0.54831135561607547884e-2_f64 * t86037 * t94954 * t29734 * t27644 + 0.54831135561607547883e-2_f64 * t94963 * t94881 + 0.54831135561607547884e-2_f64 * t94963 * t94847 - t94885 - 0.10966227112321509577e-1_f64 * t24589 * t27550 * t1751 * t3247 * t3961 + t94889 - 0.8529287754027840782e-2_f64 * t94796 * t27550 * t94797 * t17686 + t94891 + t94901;
    t103659
}
