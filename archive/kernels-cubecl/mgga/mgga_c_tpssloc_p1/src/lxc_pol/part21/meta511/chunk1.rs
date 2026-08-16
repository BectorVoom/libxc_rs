//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2158/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2158<F: Float>(t225: F, t5849: F, t1603: F, t4657: F, t1634: F, t4693: F, t3174: F, t5851: F, t17183: F, t977: F, t17178: F, t2979: F) -> (F, F, F, F, F, F) {
    let t17575 = t5849 * t225;
    let t17579 = t1603 * t4657;
    let t17582 = t1634 * t4693;
    let t17583 = t3174 * t17582;
    let t17588 = t5851 * t225;
    let t17593 = t977 * t17183;
    let t17596 = t2979 * t17178;
    (t17575, t17579, t17583, t17588, t17593, t17596)
}
