//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2490/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2490<F: Float>(t120: F, t13170: F, t13231: F, t13258: F, t41107: F, t4250: F, t13244: F, t242: F, t812: F, t841: F, t1484: F, t2678: F) -> (F, F, F, F, F, F) {
    let t46597 = t120 * t13170;
    let t46611 = t13258 * t13231;
    let t46616 = t41107 * t4250;
    let t46618 = t13258 * t13244;
    let t46628 = t812 * t841 * t242;
    let t46644 = t1484 * t2678;
    (t46597, t46611, t46616, t46618, t46628, t46644)
}
