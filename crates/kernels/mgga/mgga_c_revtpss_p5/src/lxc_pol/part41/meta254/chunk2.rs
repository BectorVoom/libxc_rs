//! MGGA_C_REVTPSS lxc pol — lxc_pol part 41 (v4rho3tau_4) CSE chunk 973/1497 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk973<F: Float>(t670: F, t8295: F, t117: F, t8273: F, t1459: F, t1461: F, t2187: F, t2189: F, t572: F, t573: F, t8289: F, t1843: F, t2178: F) -> (F, F, F, F) {
    let t8296 = t8295 * t670;
    let t8299 = t117 * t8273;
    let t8302 = F::new(3.0) * t1459 * t2189 + F::new(3.0) * t1461 * t2187 + F::new(6.0) * t572 * t8296 + F::new(3.0) * t572 * t8299 + t573 * t8289;
    let t8353 = t1843 * t2178;
    (t8296, t8299, t8302, t8353)
}
