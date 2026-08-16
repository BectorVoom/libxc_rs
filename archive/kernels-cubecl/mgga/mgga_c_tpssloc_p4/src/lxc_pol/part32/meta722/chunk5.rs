//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 2304/2369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2304<F: Float>(t1209: F, t1751: F, t17686: F, t24589: F, t24812: F, t24813: F, t27490: F, t27491: F, t27496: F, t27497: F, t27501: F, t27536: F, t27550: F, t27644: F, t29734: F, t3247: F, t3502: F, t3961: F, t5012: F, t7373: F, t86037: F, t94796: F, t94797: F, t94847: F, t94881: F, t94885: F, t94889: F, t94891: F, t94901: F, t94954: F, t94963: F) -> F {
    let t103659 = -F::cast_from(0.16449340668482264365e-1_f64) * t24812 * t27496 * t27490 * t5012 + F::cast_from(0.3289868133696452873e-1_f64) * t24812 * t24813 * t3502 * t1751 * t27491 - F::cast_from(0.16449340668482264365e-1_f64) * t24812 * t24813 * t1209 * t1751 * t27497 - F::cast_from(0.16449340668482264365e-1_f64) * t7373 * t27536 * t27501 + F::cast_from(0.54831135561607547884e-2_f64) * t86037 * t94954 * t29734 * t27644 + F::cast_from(0.54831135561607547883e-2_f64) * t94963 * t94881 + F::cast_from(0.54831135561607547884e-2_f64) * t94963 * t94847 - t94885 - F::cast_from(0.10966227112321509577e-1_f64) * t24589 * t27550 * t1751 * t3247 * t3961 + t94889 - F::cast_from(0.8529287754027840782e-2_f64) * t94796 * t27550 * t94797 * t17686 + t94891 + t94901;
    t103659
}
