//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 879/1124 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk879<F: Float>(t4680: F, t7426: F, t8476: F, t30937: F, t8450: F, t30638: F, t10098: F, t8462: F, t8653: F, t30655: F, t30407: F, t30408: F, t30409: F, t495: F, t30402: F, t506: F) -> (F, F, F, F, F, F, F, F) {
    let t34556 = t7426 * t4680 * t8476;
    let t34561 = t30937 * t8450;
    let t34566 = 35.0 / 216.0 * t30638;
    let t34569 = t10098 * t8462;
    let t34570 = t34569 * t8653;
    let t34575 = 0.42874018118069736972e-3 * t30655;
    let t34578 = t30407 * t30408 * t30409 * t495;
    let t34582 = t30407 * t30402 * t30409 * t506;
    (t34556, t34561, t34566, t34569, t34570, t34575, t34578, t34582)
}
