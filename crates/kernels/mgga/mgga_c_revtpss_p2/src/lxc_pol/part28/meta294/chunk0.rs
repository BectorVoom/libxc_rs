//! MGGA_C_REVTPSS lxc pol — lxc_pol part 28 (v4rho3sigma_3) CSE chunk 1284/2277 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1284<F: Float>(t550: F, t9721: F, t268: F, t9718: F, t64: F, t8779: F, t159: F, t535: F, t2236: F, t65: F, t235: F, t1389: F, t3964: F) -> (F, F, F, F, F, F, F) {
    let t9722 = t9721 * t550;
    let t9723 = t9722 * t268;
    let t9725 = F::cast_from(0.20082057720118594944e-6_f64) * t9718 * t9723;
    let t9726 = t64 * t8779;
    let t9727 = t9726 * t159;
    let t9729 = F::new(455.0) / F::new(1296.0) * t9727 * t535;
    let t9731 = F::new(1.0) / t65 / t2236;
    let t9732 = t235 * t9731;
    let t9735 = F::cast_from(0.81322168495418382223e-4_f64) * t3964 * t9732 * t1389;
    (t9723, t9725, t9727, t9729, t9731, t9732, t9735)
}
