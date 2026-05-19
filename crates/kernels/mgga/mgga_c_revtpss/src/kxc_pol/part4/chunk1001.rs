//! MGGA_C_REVTPSS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 1001/1428 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_kxc_pol_part4_v3rho3_1_chunk1001<F: Float>(t10547: F, t2798: F, t760: F, t9323: F, t9318: F, t2251: F, t750: F, t2611: F, t2398: F, t2615: F, t2609: F, t717: F) -> (F, F, F, F, F, F) {
    let t10548 = t2798 * t10547;
    let t10552 = F::cast_from(0.51947577317044391277e2_f64) * t760 * t9323;
    let t10554 = F::cast_from(0.35089341735807877242e1_f64) * t760 * t9318;
    let t10555 = t750 * t2251;
    let t10556 = t2611 * t10555;
    let t10561 = t2398 * t2615;
    let t10563 = t717 * t2609;
    (t10548, t10552, t10554, t10556, t10561, t10563)
}
