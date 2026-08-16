//! MGGA_C_REVTPSS lxc pol — lxc_pol part 33 (v4rho3sigma_8) CSE chunk 1275/2275 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1275<F: Float>(t9484: F, t9543: F, t520: F, t512: F, t1331: F, t3857: F, t2619: F, t3825: F, t1333: F, t3863: F, t2626: F, t676: F) -> (F, F, F, F, F, F, F) {
    let t9544 = t9484 + t9543;
    let t9545 = t520 * t9544;
    let t9546 = t512 * t9545;
    let t9559 = t3857 * t1331;
    let t9566 = t3825 * t2619;
    let t9569 = F::cast_from(60.0_f64) * t3857 * t1333;
    let t9570 = t3863 * t1331;
    let t9572 = t676 * t2626;
    (t9544, t9546, t9559, t9566, t9569, t9570, t9572)
}
