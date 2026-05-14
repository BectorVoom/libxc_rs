//! MGGA_C_PKZB lxc pol — lxc_pol part 10 (v4rho4_2) CSE chunk 1197/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part10_v4rho4_2_chunk1197<F: Float>(t2815: F, t5873: F, t1954: F, t2826: F, t1100: F, t5498: F, t1976: F, t1095: F, t5871: F, t1937: F, t2793: F, t1083: F, t1899: F, t1088: F, t5870: F, t5490: F) -> (F, F, F, F, F, F, F, F, F) {
    let t21134 = t2815 * t5873;
    let t21143 = t2826 * t1954;
    let t21146 = t1100 * t5498;
    let t21156 = t2826 * t1976;
    let t21173 = t5871 * t1095;
    let t21179 = t2793 * t1937;
    let t21184 = t1899 * t1083;
    let t21203 = t1088 * t5870;
    let t21212 = t1100 * t5490;
    (t21134, t21143, t21146, t21156, t21173, t21179, t21184, t21203, t21212)
}
