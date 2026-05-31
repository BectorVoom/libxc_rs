//! MGGA_C_REVTPSS lxc pol — lxc_pol part 52 (v4rho2sigma2_7) CSE chunk 1288/1292 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part52_v4rho2sigma2_7_chunk1288<F: Float>(t127495: F, t129095: F, t129097: F, t129099: F, t129103: F, t129107: F, t129109: F, t129111: F, t1461: F, t34011: F, t34014: F, t34341: F, t7324: F, t8127: F, t8616: F) -> F {
    let t129112 = F::cast_from(3.0_f64) * t1461 * t34341 + F::cast_from(3.0_f64) * t7324 * t8127 + t127495 + t129095 + t129097 + t129099 + t129103 + t129107 + t129109 + t129111 + t34011 + t34014 + t8616;
    t129112
}
