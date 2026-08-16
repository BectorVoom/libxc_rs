//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3227/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3227<F: Float>(t15071: F, t1940: F, t2403: F, t40084: F, t40088: F, t40099: F, t40103: F, t40115: F, t4556: F, t61197: F, t61198: F, t61199: F, t61200: F, t61202: F, t61203: F, t61209: F) -> F {
    let t61210 = -F::cast_from(2.0_f64) * t15071 * t1940 * t4556 - F::cast_from(6.0_f64) * t2403 * t4556 * t61203 + t40084 + t40088 + t40099 + t40103 - t40115 + t61197 - t61198 + t61199 + t61200 + t61202 + t61209;
    t61210
}
