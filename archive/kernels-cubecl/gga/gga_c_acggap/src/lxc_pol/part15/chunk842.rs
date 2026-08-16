//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 842/1278 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk842<F: Float>(t8185: F, t8190: F, t8192: F, t8193: F, t8195: F, t8205: F, t8209: F, t8219: F, t8220: F, t8221: F, t8232: F, t8772: F, t8829: F, t9661: F, t9664: F, t9667: F, t9671: F, t9675: F, t9677: F) -> F {
    let t9922 = -t8185 + t8190 + t8192 + t8193 - t8195 - F::cast_from(0.916875e-1_f64) * t9661 + F::cast_from(0.4584375e-1_f64) * t9664 + F::cast_from(0.305625e-1_f64) * t9667 + F::cast_from(0.42874018118069736972e-2_f64) * t9671 - t8205 + t8209 + t8219 + t8220 - t8221 - F::cast_from(0.305625e-1_f64) * t8772 + F::cast_from(0.31448092289604152069e-3_f64) * t9675 - F::cast_from(0.16809375e0_f64) * t8829 + t8232 - F::cast_from(0.68598428988911579156e-2_f64) * t9677;
    t9922
}
