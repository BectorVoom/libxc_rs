//! MGGA_C_REVTPSS lxc pol — lxc_pol part 36 (v4rho3sigma_11) CSE chunk 807/1378 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part36_v4rho3sigma_11_chunk807<F: Float>(t9484: F, t9543: F, t520: F, t512: F, t1333: F, t3857: F, t2626: F, t676: F, t3869: F, t2434: F, t762: F, t186: F, t685: F, t793: F) -> (F, F, F, F, F, F, F, F) {
    let t9544 = t9484 + t9543;
    let t9545 = t520 * t9544;
    let t9546 = t512 * t9545;
    let t9569 = F::cast_from(60.0_f64) * t3857 * t1333;
    let t9572 = t676 * t2626;
    let t9574 = F::cast_from(0.32530743900905219526e-1_f64) * t3869 * t9572;
    let t9575 = t2434 * t762;
    let t9577 = F::cast_from(0.21687162600603479684e-1_f64) * t3869 * t9575;
    let t9586 = t685 * t793 * t186;
    (t9544, t9546, t9569, t9572, t9574, t9575, t9577, t9586)
}
