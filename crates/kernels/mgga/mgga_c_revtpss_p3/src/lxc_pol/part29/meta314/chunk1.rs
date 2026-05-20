//! MGGA_C_REVTPSS lxc pol — lxc_pol part 29 (v4rho3sigma_4) CSE chunk 1217/2049 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1217<F: Float>(t2629: F, t9866: F, t9575: F, t9572: F, t177: F, t2390: F, t762: F, t760: F, t9419: F, t2516: F, t2523: F, t9387: F) -> (F, F, F, F, F, F, F) {
    let t10582 = F::cast_from(0.48159733137676571078e0_f64) * t2629 * t9866;
    let t10584 = F::cast_from(0.21687162600603479684e-1_f64) * t2629 * t9575;
    let t10586 = F::cast_from(0.32530743900905219526e-1_f64) * t2629 * t9572;
    let t10587 = t2390 * t177;
    let t10588 = t10587 * t762;
    let t10592 = F::cast_from(0.10389515463408878255e3_f64) * t760 * t9419;
    let t10593 = t2523 * t2516;
    let t10596 = F::cast_from(0.5848223622634646207e0_f64) * t760 * t9387;
    (t10582, t10584, t10586, t10588, t10592, t10593, t10596)
}
