//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 2865/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2865<F: Float>(t1235: F, t3661: F, t371: F, t676: F, t1236: F, t2434: F, t3671: F, t3672: F, t12625: F, t458: F, t456: F, t225: F) -> (F, F, F, F, F) {
    let t44823 = t1235 * t371 * t676 * t3661;
    let t44829 = t1235 * t371 * t2434 * t1236;
    let t44838 = t3671 * t371 * t676 * t3672;
    let t44841 = F::new(1.0) / t12625 / t458;
    let t44842 = t456 * t44841;
    let t44843 = t44842 * t225;
    (t44823, t44829, t44838, t44842, t44843)
}
