//! MGGA_C_REVTPSS lxc pol — lxc_pol part 30 (v4rho3sigma_5) CSE chunk 2212/2270 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2212<F: Float>(t101214: F, t2122: F, t101172: F, t101176: F, t101182: F, t101187: F, t101399: F, t2123: F, t25162: F, t26749: F, t26755: F, t26792: F, t28105: F, t28109: F, t7566: F, t7706: F, t96792: F, t96810: F) -> F {
    let t104226 = t2122 * t101214;
    let t104249 = -F::new(10.0) * t26792 * t101399 - F::new(10.0) / F::new(3.0) * t25162 * t104226 - F::new(5.0) / F::new(3.0) * t96810 * t7706 + F::new(5.0) / F::new(6.0) * t96792 * t7706 + F::new(5.0) / F::new(3.0) * t26755 * t28105 + F::new(5.0) / F::new(3.0) * t26755 * t28109 + F::new(5.0) / F::new(3.0) * t26749 * t28105 + F::new(5.0) / F::new(6.0) * t7566 * t101172 + F::new(5.0) / F::new(3.0) * t7566 * t101176 + F::new(5.0) / F::new(3.0) * t26749 * t28109 + F::new(5.0) / F::new(6.0) * t7566 * t101182 + t101187 * t2123 / F::new(3.0);
    t104249
}
