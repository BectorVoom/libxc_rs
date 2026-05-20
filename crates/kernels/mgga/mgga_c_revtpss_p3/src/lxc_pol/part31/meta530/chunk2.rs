//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 1909/2259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1909<F: Float>(t1928: F, t25099: F, t25157: F, t25162: F, t25164: F, t28116: F, t28119: F, t28127: F, t28133: F, t28138: F, t28141: F, t28147: F, t28151: F, t28154: F, t6958: F, t6960: F, t6963: F, t6974: F, t6978: F, t7706: F, t7709: F, t7716: F, t7720: F) -> F {
    let t28157 = t28116 * t1928 / F::new(3.0) + t28119 * t1928 / F::new(3.0) + t7709 * t6974 / F::new(3.0) + t7709 * t6978 / F::new(3.0) + F::new(5.0) / F::new(6.0) * t28127 * t6960 + t6963 * t7716 / F::new(3.0) + F::new(5.0) / F::new(6.0) * t6958 * t28133 + t6963 * t7720 / F::new(3.0) + F::new(5.0) / F::new(6.0) * t28138 * t6960 + t28141 * t1928 / F::new(3.0) + F::new(5.0) / F::new(6.0) * t25099 * t7706 - F::new(5.0) * t25157 * t28147 - F::new(5.0) / F::new(3.0) * t25162 * t28151 - F::new(5.0) / F::new(3.0) * t28154 * t25164;
    t28157
}
