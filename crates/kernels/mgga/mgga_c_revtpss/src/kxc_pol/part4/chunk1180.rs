//! MGGA_C_REVTPSS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 1180/1428 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_kxc_pol_part4_v3rho3_1_chunk1180<F: Float>(t1531: F, t37: F, t2612: F, t4392: F, t72: F, t757: F, t14425: F, t150: F, t190: F, t10608: F, t2258: F, t4402: F) -> (F, F, F, F, F) {
    let t14613 = t37 * t1531;
    let t14615 = F::new(12.0) * t14613 * t2612;
    let t14616 = t4392 * t72;
    let t14618 = F::cast_from(0.36622894612013090108e-3_f64) * t14616 * t757;
    let t14619 = t150 * t14425;
    let t14620 = t14619 * t190;
    let t14621 = F::cast_from(0.23392894490538584828e1_f64) * t10608;
    let t14622 = t4402 * t2258;
    (t14615, t14618, t14620, t14621, t14622)
}
