//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1068/1171 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk1068<F: Float>(t27903: F, t44544: F, t7703: F, t95890: F, t1094: F, t4923: F, t1014: F, t27859: F, t27811: F, t61287: F, t4981: F, t982: F, t990: F, t26757: F, t27832: F, t26714: F, t8030: F) -> (F, F, F, F, F, F, F, F, F) {
    let t96150 = t7703 * t44544 * t27903;
    let t96173 = t7703 * t95890;
    let t96210 = t4923 * t1094;
    let t96217 = t1014 * t27859;
    let t96218 = 0.22109259259259259258e-2 * t96217;
    let t96221 = t27811 * t61287;
    let t96227 = t4981 * t982 * t990;
    let t96231 = 0.15445601851851851852e-3 * t27832 * t26757;
    let t96238 = 0.46336805555555555556e-3 * t8030 * t26714;
    (t96150, t96173, t96210, t96217, t96218, t96221, t96227, t96231, t96238)
}
