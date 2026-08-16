//! MGGA_C_REVTPSS lxc pol — lxc_pol part 30 (v4rho3sigma_5) CSE chunk 1273/2270 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1273<F: Float>(t3850: F, t72: F, t757: F, t2619: F, t3825: F, t1333: F, t3857: F, t2626: F, t676: F, t3869: F, t2434: F, t762: F) -> (F, F, F, F, F, F) {
    let t9563 = t3850 * t72;
    let t9564 = t9563 * t757;
    let t9566 = t3825 * t2619;
    let t9569 = F::cast_from(60.0_f64) * t3857 * t1333;
    let t9572 = t676 * t2626;
    let t9574 = F::cast_from(0.32530743900905219526e-1_f64) * t3869 * t9572;
    let t9575 = t2434 * t762;
    (t9564, t9566, t9569, t9572, t9574, t9575)
}
