//! MGGA_C_REVTPSS lxc pol — lxc_pol part 34 (v4rho3sigma_9) CSE chunk 1150/1341 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part34_v4rho3sigma_9_chunk1150<F: Float>(t25220: F, t25232: F, t25243: F, t28330: F, t28333: F, t28335: F, t28336: F, t29616: F, t29618: F, t29620: F, t29635: F) -> F {
    let t29636 = t25220 - t25232 + t25243 + t28330 + F::new(0.85748036236139473944e-3) * t29616 + F::new(0.34299214494455789578e-2) * t29618 - F::new(0.42874018118069736972e-3) * t29620 - t28335 + t28336 + t28333 + t29635;
    t29636
}
