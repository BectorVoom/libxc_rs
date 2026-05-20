//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2596/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2596<F: Float>(t18544: F, t2398: F, t14440: F, t4311: F, t14386: F, t4305: F, t177: F, t18550: F, t762: F, t123: F, t2630: F, t5941: F) -> (F, F, F, F, F) {
    let t61178 = t2398 * t18544;
    let t61180 = t4311 * t14440;
    let t61201 = t14386 * t4305;
    let t61239 = t18550 * t177 * t762;
    let t61247 = t5941 * t123 * t2630;
    (t61178, t61180, t61201, t61239, t61247)
}
