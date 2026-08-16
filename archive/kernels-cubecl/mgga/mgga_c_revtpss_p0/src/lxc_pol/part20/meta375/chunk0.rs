//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1358/1798 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1358<F: Float>(t40360: F, t839: F, t10639: F, t221: F, t2484: F, t2485: F, t10820: F, t2652: F, t231: F, t40262: F, t10841: F, t10845: F) -> (F, F, F, F, F) {
    let t40361 = t40360 * t839;
    let t40365 = t2484 * t2485 * t221 * t10639;
    let t40367 = t2652 * t10820;
    let t40369 = t40262 * t231;
    let t40374 = t10845 * t10841;
    (t40361, t40365, t40367, t40369, t40374)
}
