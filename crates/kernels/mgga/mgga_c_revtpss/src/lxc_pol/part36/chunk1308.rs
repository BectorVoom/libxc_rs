//! MGGA_C_REVTPSS lxc pol — lxc_pol part 36 (v4rho3sigma_11) CSE chunk 1308/1378 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part36_v4rho3sigma_11_chunk1308<F: Float>(t106080: F, t106082: F, t106090: F, t106102: F, t113222: F, t113226: F, t113228: F, t113230: F, t113232: F, t113235: F, t113237: F, t93021: F, t99091: F, t99113: F) -> F {
    let t113240 = -t93021 - F::new(0.76230004213927992339e-4) * t106080 - F::new(7.0) / F::new(16.0) * t106082 - t113222 / F::new(4.0) + F::new(7.0) / F::new(48.0) * t106090 - F::new(0.18292914397043087774e-2) * t99091 + F::new(0.25724410870841842184e-1) * t113226 + F::new(0.25724410870841842183e-2) * t113228 - F::new(0.42874018118069736972e-3) * t113230 - F::new(0.51448821741683684367e-1) * t113232 - F::new(0.27107389498472794076e-4) * t99113 - F::new(0.42874018118069736972e-3) * t113235 - F::new(0.25724410870841842183e-2) * t113237 - F::new(0.85748036236139473943e-3) * t106102;
    t113240
}
