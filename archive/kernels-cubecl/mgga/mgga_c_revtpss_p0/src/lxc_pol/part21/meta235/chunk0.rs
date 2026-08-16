//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 1386/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1386<F: Float>(t2782: F, t5737: F, t1883: F, t72: F, t686: F, t4101: F, t225: F, t3999: F) -> (F, F, F, F, F) {
    let t5738 = t2782 * t5737;
    let t5740 = t1883 * t72;
    let t5741 = t5740 * t686;
    let t5742 = t4101 * t5741;
    let t5744 = t225 * t3999;
    (t5738, t5740, t5741, t5742, t5744)
}
