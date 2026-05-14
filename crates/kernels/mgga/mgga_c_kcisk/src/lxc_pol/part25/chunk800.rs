//! MGGA_C_KCISK lxc pol — lxc_pol part 25 (v4rho3sigma_5) CSE chunk 800/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part25_v4rho3sigma_5_chunk800<F: Float>(t10568: F, t25: F, t5033: F, t1773: F, t1769: F, t4984: F, t1765: F, t4995: F, t657: F, t963: F, t397: F, t662: F, t656: F, t1782: F, t4893: F) -> (F, F, F, F, F, F, F) {
    let t10761 = 0.28842592592592592592e-1 * t10568;
    let t10784 = t25 * t5033;
    let t10785 = t1773 * t10784;
    let t10787 = t4984 * t1769;
    let t10789 = t1765 * t4995;
    let t10791 = t963 * t657;
    let t10793 = t397 * t10791 * t662;
    let t10795 = 0.19989765240197019125e-1 * t656 * t10793;
    let t10798 = t4893 * t1782;
    (t10761, t10785, t10787, t10789, t10791, t10795, t10798)
}
