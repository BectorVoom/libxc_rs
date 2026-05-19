//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 1249/1278 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk1249<F: Float>(t32823: F, t32824: F, t35635: F, t35643: F, t35648: F, t35653: F, t35662: F, t35664: F, t37636: F, t37639: F, t37646: F, t40083: F, t40086: F, t40089: F, t40092: F, t40095: F, t40099: F, t40101: F) -> F {
    let t41938 = t37636 - t37639 + F::cast_from(0.31448092289604152069e-2_f64) * t35635 - F::new(7.0) / F::new(36.0) * t40083 - F::new(0.4584375e-1) * t40086 - F::new(0.916875e-1) * t40089 - F::cast_from(0.42874018118069736972e-2_f64) * t40092 + F::cast_from(0.21437009059034868486e-2_f64) * t40095 - t32823 + t32824 + F::new(13.0) / F::new(24.0) * t35643 - t37646 - t35648 + t35653 + F::cast_from(0.21437009059034868486e-3_f64) * t40099 - F::cast_from(0.10289764348336736873e0_f64) * t40101 - F::cast_from(0.90035438047946447644e-1_f64) * t35662 - F::cast_from(0.45351183609335988441e-1_f64) * t35664;
    t41938
}
