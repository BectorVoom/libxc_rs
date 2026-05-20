//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 649/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk649<F: Float>(t2327: F, t94: F, t1310: F, t670: F, t112: F, t2289: F, t625: F, t666: F, t111: F, t654: F) -> (F, F, F, F, F) {
    let t2328 = t94 * t2327;
    let t2331 = t1310 * t670;
    let t2335 = F::new(11.0) / F::new(9.0) * t2289 * t112;
    let t2336 = t625 * t666;
    let t2339 = F::new(1.0) / t654 / t111;
    (t2328, t2331, t2335, t2336, t2339)
}
