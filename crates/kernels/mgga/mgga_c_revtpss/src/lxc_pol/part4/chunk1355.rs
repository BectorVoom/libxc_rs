//! MGGA_C_REVTPSS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 1355/1428 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part4_v3rho3_1_chunk1355<F: Float>(t16892: F, t16708: F, t16710: F, t16717: F, t16722: F, t16735: F, t16740: F, t16744: F, t16908: F, t16927: F, t16931: F, t16933: F) -> (F, F) {
    let t17131 = F::new(0.22076e0) * t16892;
    let t17140 = F::new(0.13418888888888888889e0) * t16708;
    let t17148 = F::new(0.36793333333333333333e-1) * t16908 + F::new(0.16504875e0) * t16927 - F::new(0.40256666666666666667e0) * t16710 + t17140 + F::new(0.36793333333333333334e-1) * t16931 + F::new(0.258925e1) * t16933 - F::new(0.12077e1) * t16722 + F::new(0.12077e1) * t16740 + F::new(0.60385e0) * t16744 + F::new(0.181155e1) * t16735 + F::new(0.33547222222222222222e0) * t16717;
    (t17131, t17148)
}
