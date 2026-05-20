//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3610/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3610<F: Float>(t43865: F, t43888: F, t43890: F, t43892: F, t56230: F, t56236: F, t68389: F, t68393: F, t68397: F, t68399: F, t68454: F, t68456: F, t68459: F) -> F {
    let t68461 = -F::new(2.0) / F::new(9.0) * t56230 - F::new(56.0) / F::new(81.0) * t56236 - F::new(2.0) / F::new(9.0) * t68389 + t68393 / F::new(3.0) - F::new(4.0) / F::new(9.0) * t68397 + F::new(8.0) / F::new(27.0) * t68399 - F::new(8.0) / F::new(81.0) * t43865 - F::new(56.0) / F::new(81.0) * t43888 + F::new(4.0) / F::new(27.0) * t43890 + F::new(8.0) / F::new(27.0) * t43892 - F::new(8.0) / F::new(9.0) * t68454 - F::new(4.0) / F::new(3.0) * t68456 + F::new(2.0) * t68459;
    t68461
}
