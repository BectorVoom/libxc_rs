//! MGGA_C_REVTPSS lxc pol — lxc_pol part 42 (v4rho3tau_5) CSE chunk 959/1363 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk959<F: Float>(t9484: F, t9543: F, t520: F, t512: F, t1331: F, t3857: F, t2619: F, t3825: F, t1333: F, t3863: F, t2626: F, t676: F, t3869: F, t2434: F, t762: F, t3860: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t9544 = t9484 + t9543;
    let t9545 = t520 * t9544;
    let t9546 = t512 * t9545;
    let t9559 = t3857 * t1331;
    let t9566 = t3825 * t2619;
    let t9569 = 60.0 * t3857 * t1333;
    let t9570 = t3863 * t1331;
    let t9572 = t676 * t2626;
    let t9574 = 0.32530743900905219526e-1 * t3869 * t9572;
    let t9575 = t2434 * t762;
    let t9577 = 0.21687162600603479684e-1 * t3869 * t9575;
    let t9578 = t3860 * t1331;
    (t9544, t9546, t9559, t9566, t9569, t9570, t9572, t9574, t9575, t9577, t9578)
}
