//! MGGA_C_REVTPSS lxc pol — lxc_pol part 26 (v4rho3sigma_1) CSE chunk 822/1225 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part26_v4rho3sigma_1_chunk822<F: Float>(t760: F, t9323: F, t9318: F, t2251: F, t750: F, t2611: F, t10467: F, t162: F, t187: F, t2398: F, t2615: F, t2609: F, t717: F) -> (F, F, F, F, F, F) {
    let t10552 = F::new(0.51947577317044391277e2) * t760 * t9323;
    let t10554 = F::new(0.35089341735807877242e1) * t760 * t9318;
    let t10555 = t750 * t2251;
    let t10556 = t2611 * t10555;
    let t10557 = F::new(36.0) * t10556;
    let t10558 = t10467 * t162;
    let t10560 = F::new(0.19751673498613801407e-1) * t10558 * t187;
    let t10561 = t2398 * t2615;
    let t10562 = F::new(24.0) * t10561;
    let t10563 = t717 * t2609;
    (t10552, t10554, t10557, t10560, t10562, t10563)
}
