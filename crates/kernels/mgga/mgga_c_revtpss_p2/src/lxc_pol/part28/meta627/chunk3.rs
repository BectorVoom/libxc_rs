//! MGGA_C_REVTPSS lxc pol — lxc_pol part 28 (v4rho3sigma_3) CSE chunk 2250/2277 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2250<F: Float>(t4237: F, t644: F, t77: F, t1497: F, t2311: F, t4241: F, t640: F, t13420: F, t84: F, t25099: F, t25106: F, t28086: F, t28090: F, t28105: F, t28109: F, t6958: F, t6963: F, t7706: F, t92644: F, t92702: F) -> F {
    let t101156 = t77 * t4237 * t644;
    let t101172 = t77 * t2311 * t1497;
    let t101176 = t77 * t640 * t4241;
    let t101182 = t77 * t84 * t13420;
    let t101185 = F::new(2.0) / F::new(3.0) * t6963 * t28086 + F::new(5.0) / F::new(3.0) * t6958 * t101156 + F::new(2.0) / F::new(3.0) * t6963 * t28090 + F::new(5.0) / F::new(3.0) * t92702 * t7706 + F::new(5.0) / F::new(6.0) * t92644 * t7706 + F::new(5.0) / F::new(3.0) * t25106 * t28105 + F::new(5.0) / F::new(3.0) * t25106 * t28109 + F::new(5.0) / F::new(3.0) * t25099 * t28105 + F::new(5.0) / F::new(6.0) * t6958 * t101172 + F::new(5.0) / F::new(3.0) * t6958 * t101176 + F::new(5.0) / F::new(3.0) * t25099 * t28109 + F::new(5.0) / F::new(6.0) * t6958 * t101182;
    t101185
}
