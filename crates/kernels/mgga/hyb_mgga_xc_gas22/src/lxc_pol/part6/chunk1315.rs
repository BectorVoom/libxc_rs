//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1315/1345 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1315<F: Float>(t11391: F, t9489: F, t4576: F, t7785: F, t22754: F, t26429: F, t2829: F, t30752: F, t30760: F, t30764: F, t30767: F, t30772: F, t30777: F, t30781: F, t30784: F, t30787: F, t30790: F, t3753: F, t3757: F, t7806: F, t7811: F, t9533: F, t9542: F, t9558: F, t9587: F) -> (F, F, F) {
    let t30793 = t11391 * t9489;
    let t30796 = t4576 * t7785;
    let t30799 = -88.0 / 27.0 * t2829 * t30752 + 64.0 / 3.0 * t26429 * t30760 - 1936.0 / 243.0 * t3757 * t30764 - 32.0 / 3.0 * t22754 * t30767 + 5632.0 / 2187.0 * t9587 * t30772 - 4096.0 / 729.0 * t9587 * t30777 - 2560.0 / 243.0 * t3753 * t30781 - 112.0 / 3.0 * t9558 * t30784 + 32.0 * t7806 * t30787 - 800.0 / 9.0 * t9533 * t30790 + 4000.0 / 9.0 * t9542 * t30793 - 352.0 / 27.0 * t7811 * t30796;
    (t30793, t30796, t30799)
}
