//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 2226/2259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2226<F: Float>(t101385: F, t101391: F, t28078: F, t28081: F, t28086: F, t28105: F, t28109: F, t28112: F, t28116: F, t28119: F, t28127: F, t29538: F, t6974: F, t6978: F, t7706: F, t7709: F, t7720: F) -> F {
    let t108854 = F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t7709 * t28078 + F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t7709 * t28081 + F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t28112 * t7720 + F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t29538 * t6974 + F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t29538 * t6978 + F::cast_from(5.0_f64) / F::cast_from(3.0_f64) * t101385 * t7706 + F::cast_from(5.0_f64) / F::cast_from(3.0_f64) * t101391 * t7706 + F::cast_from(5.0_f64) / F::cast_from(3.0_f64) * t28127 * t28105 + F::cast_from(5.0_f64) / F::cast_from(3.0_f64) * t28127 * t28109 + F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t28116 * t7720 + F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t28119 * t7720 + F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t7709 * t28086;
    t108854
}
