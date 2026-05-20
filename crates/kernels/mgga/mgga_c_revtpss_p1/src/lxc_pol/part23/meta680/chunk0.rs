//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2420/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2420<F: Float>(t1121: F, t13045: F, t606: F, t221: F, t461: F, t462: F, t624: F, t1250: F, t1235: F, t1236: F, t2434: F, t371: F) -> (F, F, F, F) {
    let t44737 = t13045 * t1121;
    let t44738 = t44737 * t606;
    let t44797 = F::new(5.0) / F::new(486.0) * t461 * t221 * t624 * t462;
    let t44799 = t1250 * t606;
    let t44829 = t1235 * t371 * t2434 * t1236;
    (t44738, t44797, t44799, t44829)
}
