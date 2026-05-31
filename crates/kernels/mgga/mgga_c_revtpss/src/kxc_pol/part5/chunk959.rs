//! MGGA_C_REVTPSS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 959/1422 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_kxc_pol_part5_v3rho3_2_chunk959<F: Float>(t730: F, t9434: F, t2552: F, t722: F, t164: F, t172: F, t2555: F, t177: F, t9367: F, t9368: F, t9371: F, t701: F, t9275: F) -> (F, F, F, F, F, F) {
    let t9525 = t9434 * t730;
    let t9529 = F::cast_from(1.0_f64) / t2552 / t722;
    let t9530 = t164 * t9529;
    let t9532 = F::cast_from(1.0_f64) / t2555 / t172;
    let t9533 = t9434 * t9532;
    let t9536 = t177 * t9367;
    let t9537 = t9368 * t9371;
    let t9540 = t9275 * t701;
    (t9525, t9530, t9533, t9536, t9537, t9540)
}
