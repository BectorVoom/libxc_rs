//! MGGA_C_REVTPSS lxc pol — lxc_pol part 55 (v4rho2sigma2_10) CSE chunk 1281/1306 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part55_v4rho2sigma2_10_chunk1281<F: Float>(t104115: F, t111734: F, t128198: F, t128200: F, t128204: F, t128211: F, t128219: F, t128223: F, t128225: F, t128228: F, t128231: F, t128235: F, t128236: F, t2056: F, t5787: F, t8897: F) -> F {
    let t130907 = -F::cast_from(2.0_f64) * t104115 * t2056 - F::cast_from(2.0_f64) * t111734 * t2056 + t5787 * t8897 + t128198 - t128200 - t128204 - t128211 - t128219 + t128223 + t128225 - t128228 + t128231 - t128235 - t128236;
    t130907
}
