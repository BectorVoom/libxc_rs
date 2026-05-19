//! MGGA_C_REVTPSS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 1162/1422 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_kxc_pol_part5_v3rho3_2_chunk1162<F: Float>(t5941: F, t72: F, t757: F, t10569: F, t4186: F, t4402: F, t4401: F, t177: F, t762: F, t10579: F, t14386: F, t1522: F) -> (F, F, F, F, F, F) {
    let t18555 = t5941 * t72;
    let t18556 = t18555 * t757;
    let t18557 = F::cast_from(0.18311447306006545054e-3_f64) * t18556;
    let t18558 = F::cast_from(0.24415263074675393405e-3_f64) * t10569;
    let t18559 = t4402 * t4186;
    let t18561 = F::new(24.0) * t4401 * t18559;
    let t18562 = t5941 * t177;
    let t18563 = t18562 * t762;
    let t18564 = F::cast_from(0.5848223622634646207e0_f64) * t18563;
    let t18565 = F::cast_from(0.10843581300301739842e-1_f64) * t10579;
    let t18567 = F::new(8.0) * t14386 * t1522;
    (t18557, t18558, t18561, t18564, t18565, t18567)
}
