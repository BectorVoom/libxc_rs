//! MGGA_C_REVTPSS lxc pol — lxc_pol part 53 (v4rho2sigma2_8) CSE chunk 1216/1244 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part53_v4rho2sigma2_8_chunk1216<F: Float>(t125420: F, t125431: F, t125432: F, t125433: F, t125436: F, t125438: F, t125442: F, t125444: F, t125456: F, t125459: F, t125467: F, t2127: F, t27830: F, t7584: F, t7883: F) -> F {
    let t129298 = -t2127 * t27830 - t7584 * t7883 - F::new(2.0) * t125420 - t125431 - t125432 - F::new(2.0) * t125433 - F::new(2.0) * t125436 - F::new(2.0) * t125438 - F::new(2.0) * t125442 - F::new(2.0) * t125444 - t125456 - F::new(2.0) * t125459 - t125467;
    t129298
}
