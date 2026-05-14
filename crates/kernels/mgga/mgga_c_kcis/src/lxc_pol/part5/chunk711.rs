//! MGGA_C_KCIS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 711/1260 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part5_v3rho3_2_chunk711<F: Float>(t1396: F, t5676: F, t4123: F, t1464: F, t1948: F, t4142: F, t1014: F, t2007: F, t1984: F, t1365: F, t1930: F, t1929: F, t990: F) -> (F, F, F, F, F, F, F, F) {
    let t5677 = t1396 * t5676;
    let t5678 = t4123 * t5677;
    let t5679 = t1464 * t5678;
    let t5681 = t4142 * t1948;
    let t5684 = t1014 * t2007;
    let t5686 = t1014 * t1984;
    let t5689 = t1930 * t1365;
    let t5691 = t1929 * t990;
    (t5677, t5678, t5679, t5681, t5684, t5686, t5689, t5691)
}
