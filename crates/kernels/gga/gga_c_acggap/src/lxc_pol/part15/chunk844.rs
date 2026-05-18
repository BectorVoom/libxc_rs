//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 844/1278 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk844<F: Float>(t8247: F, t8249: F, t8252: F, t8253: F, t8254: F, t8257: F, t8268: F, t8269: F, t8271: F, t8275: F, t8276: F, t8898: F, t9713: F, t9715: F, t9717: F, t9721: F, t9725: F, t9728: F, t9731: F, t9735: F) -> F {
    let t9951 = -F::new(0.916875e-1) * t9713 - F::new(0.34299214494455789578e-2) * t9715 + F::new(0.34299214494455789578e-2) * t9717 + F::new(0.62896184579208304137e-2) * t9721 - t8247 - t8249 + F::new(0.42874018118069736972e-3) * t8898 + F::new(0.94344276868812456207e-3) * t9725 - t8252 - t8253 + t8254 + t8257 + t9728 / F::new(12.0) + t9731 / F::new(32.0) - F::new(0.4584375e-1) * t9735 - t8268 + t8269 - t8271 + t8275 - t8276;
    t9951
}
