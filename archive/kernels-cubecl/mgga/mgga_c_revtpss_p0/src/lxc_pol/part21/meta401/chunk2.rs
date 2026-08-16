//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 1862/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1862<F: Float>(t3667: F, t3678: F, t1236: F, t371: F, t676: F, t1235: F, t12627: F, t225: F) -> (F, F, F, F) {
    let t12979 = t3667 * t3678;
    let t12984 = t371 * t676 * t1236;
    let t12985 = t1235 * t12984;
    let t12987 = t12627 * t225;
    (t12979, t12984, t12985, t12987)
}
