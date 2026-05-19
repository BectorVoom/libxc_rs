//! MGGA_C_REVTPSS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 993/1422 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_kxc_pol_part5_v3rho3_2_chunk993<F: Float>(t2629: F, t9575: F, t9572: F, t760: F, t9419: F, t2516: F, t2523: F, t9387: F, t2496: F, t9372: F, t37: F, t716: F) -> (F, F, F, F, F, F, F, F) {
    let t10584 = F::cast_from(0.21687162600603479684e-1_f64) * t2629 * t9575;
    let t10586 = F::cast_from(0.32530743900905219526e-1_f64) * t2629 * t9572;
    let t10592 = F::cast_from(0.10389515463408878255e3_f64) * t760 * t9419;
    let t10593 = t2523 * t2516;
    let t10596 = F::cast_from(0.5848223622634646207e0_f64) * t760 * t9387;
    let t10597 = t2523 * t2496;
    let t10604 = F::cast_from(0.10254018858216406658e4_f64) * t760 * t9372;
    let t10605 = t37 * t716;
    (t10584, t10586, t10592, t10593, t10596, t10597, t10604, t10605)
}
