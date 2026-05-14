//! MGGA_C_REVTPSS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 1070/1286 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_kxc_pol_part5_v3rho3_2_chunk1070<F: Float>(t5941: F, t72: F, t757: F, t10569: F, t4186: F, t4402: F, t4401: F, t177: F, t762: F, t10579: F, t14386: F, t1522: F, t10566: F, t10568: F, t10577: F, t10582: F, t10584: F, t10586: F, t9514: F, t9517: F, t9521: F) -> (F, F, F, F, F, F, F) {
    let t18555 = t5941 * t72;
    let t18556 = t18555 * t757;
    let t18557 = 0.18311447306006545054e-3 * t18556;
    let t18558 = 0.24415263074675393405e-3 * t10569;
    let t18559 = t4402 * t4186;
    let t18561 = 24.0 * t4401 * t18559;
    let t18562 = t5941 * t177;
    let t18563 = t18562 * t762;
    let t18564 = 0.5848223622634646207e0 * t18563;
    let t18565 = 0.10843581300301739842e-1 * t10579;
    let t18567 = 8.0 * t14386 * t1522;
    let t18568 = t10566 - t18557 - t10568 + t18558 + t18561 - t18564 + t9514 - t9517 - t9521 + t10577 + t18565 + t10582 - t10584 - t10586 + t18567;
    (t18557, t18558, t18561, t18564, t18565, t18567, t18568)
}
