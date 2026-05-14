//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1062/1221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk1062<F: Float>(t26553: F, t755: F, t774: F, t2150: F, t8537: F, t8538: F, t153: F, t822: F, t2484: F, t26547: F, t2490: F, t2491: F, t7627: F, t160: F, t805: F, t91828: F, t91830: F, t91832: F, t91835: F, t91837: F, t91839: F, t91841: F, t91844: F) -> (F, F, F, F, F, F, F) {
    let t91847 = t755 * t26553 * t774;
    let t91850 = t8537 * t2150 * t8538;
    let t91852 = t153 * t822;
    let t91854 = t2484 * t26547;
    let t91857 = t2490 * t7627 * t2491;
    let t91859 = t805 * t160;
    let t91861 = -3.0 / 16.0 * t91828 + t91830 / 8.0 + 3.0 / 2.0 * t91832 + 15.0 / 4.0 * t91835 + 3.0 / 32.0 * t91837 - t91839 / 8.0 - t91841 / 32.0 - 3.0 / 8.0 * t91844 + 15.0 / 8.0 * t91847 + 3.0 / 16.0 * t91850 - 3.0 * t91852 - 3.0 / 4.0 * t91854 + 3.0 / 4.0 * t91857 + 9.0 / 4.0 * t91859;
    (t91847, t91850, t91852, t91854, t91857, t91859, t91861)
}
