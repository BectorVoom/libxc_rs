//! MGGA_C_REVTPSS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 903/1274 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_kxc_pol_part4_v3rho3_1_chunk903<F: Float>(t1333: F, t3857: F, t2626: F, t676: F, t3869: F, t2434: F, t762: F, t1331: F, t3860: F, t1320: F, t3855: F, t186: F, t685: F, t793: F, t1337: F, t4146: F, t565: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t9569 = 60.0 * t3857 * t1333;
    let t9572 = t676 * t2626;
    let t9574 = 0.32530743900905219526e-1 * t3869 * t9572;
    let t9575 = t2434 * t762;
    let t9577 = 0.21687162600603479684e-1 * t3869 * t9575;
    let t9578 = t3860 * t1331;
    let t9580 = t1320 * t3855;
    let t9586 = t685 * t793 * t186;
    let t9588 = 0.56968947174242584612e-3 * t1337 * t9586;
    let t9593 = 1.0 / t4146 / t565;
    (t9569, t9572, t9574, t9575, t9577, t9578, t9580, t9586, t9588, t9593)
}
