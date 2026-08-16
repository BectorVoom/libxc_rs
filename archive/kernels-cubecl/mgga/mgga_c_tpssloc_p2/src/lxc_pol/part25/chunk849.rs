//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 25 (v4rho3sigma_1) CSE chunk 849/1226 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part25_v4rho3sigma_1_chunk849<F: Float>(t1040: F, t3077: F, t2775: F, t283: F, t61: F, t10305: F, t248: F, t135: F, t3142: F, t973: F, t3147: F, t9258: F, t998: F) -> (F, F, F, F, F) {
    let t10965 = t3077 * t1040;
    let t10969 = F::cast_from(1.0_f64) / t283 / t2775;
    let t10970 = t61 * t10969;
    let t10972 = t248 * t10970 * t10305;
    let t10981 = t135 * t3142;
    let t10982 = t973 * t10981;
    let t10984 = t135 * t3147;
    let t10985 = t973 * t10984;
    let t10987 = t998 * t9258;
    (t10965, t10972, t10982, t10985, t10987)
}
