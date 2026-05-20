//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2505/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2505<F: Float>(t221: F, t461: F, t462: F, t624: F, t1250: F, t606: F, t1235: F, t3661: F, t371: F, t676: F, t1236: F, t2434: F) -> (F, F, F, F) {
    let t44797 = F::new(5.0) / F::new(486.0) * t461 * t221 * t624 * t462;
    let t44799 = t1250 * t606;
    let t44823 = t1235 * t371 * t676 * t3661;
    let t44829 = t1235 * t371 * t2434 * t1236;
    (t44797, t44799, t44823, t44829)
}
