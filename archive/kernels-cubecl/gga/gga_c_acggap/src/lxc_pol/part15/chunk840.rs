//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 840/1278 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk840<F: Float>(t8146: F, t8619: F, t8625: F, t8650: F, t8658: F, t8666: F, t8680: F, t8682: F, t8684: F, t8690: F, t8694: F, t8706: F, t8710: F, t8712: F, t8714: F, t9584: F, t9590: F, t9594: F, t9598: F, t9602: F) -> F {
    let t9892 = t8146 + F::cast_from(0.5603125e-1_f64) * t8619 + F::cast_from(7.0_f64) / F::cast_from(72.0_f64) * t8625 + F::cast_from(0.12579236915841660828e-2_f64) * t9584 + F::cast_from(0.21437009059034868486e-2_f64) * t9590 + F::cast_from(0.85748036236139473944e-3_f64) * t9594 - F::cast_from(0.31448092289604152068e-2_f64) * t9598 - F::cast_from(0.18868855373762491241e-2_f64) * t9602 + F::cast_from(0.21437009059034868486e-2_f64) * t8650 - F::cast_from(0.18868855373762491241e-2_f64) * t8658 + F::cast_from(0.41930789719472202758e-3_f64) * t8666 + F::cast_from(11.0_f64) / F::cast_from(96.0_f64) * t8680 + F::cast_from(11.0_f64) / F::cast_from(288.0_f64) * t8682 + F::cast_from(7.0_f64) / F::cast_from(36.0_f64) * t8684 - F::cast_from(0.42874018118069736972e-3_f64) * t8690 + F::cast_from(0.34299214494455789578e-2_f64) * t8694 - F::cast_from(0.34299214494455789578e-2_f64) * t8706 + F::cast_from(0.68598428988911579156e-2_f64) * t8710 + F::cast_from(0.16006300097412701803e-1_f64) * t8712 - F::cast_from(0.16006300097412701803e-1_f64) * t8714;
    t9892
}
