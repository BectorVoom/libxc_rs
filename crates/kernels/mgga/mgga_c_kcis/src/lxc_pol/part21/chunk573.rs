//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 573/1221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk573<F: Float>(t4718: F, t950: F, t931: F, t1679: F, t2988: F, t949: F, t2986: F, t2919: F, t2992: F, t4612: F, t4615: F, t4618: F, t4623: F, t274: F, t1684: F, t45: F) -> (F, F, F, F, F, F, F, F) {
    let t4719 = t4718 * t950;
    let t4721 = 1.0 * t931 * t4719;
    let t4722 = t1679 * t2988;
    let t4723 = t4722 * t949;
    let t4725 = 0.16081824322151104822e2 * t2986 * t4723;
    let t4731 = t2992 + 0.30902777777777777778e-2 * t2919 + 0.30902777777777777778e-2 * t4612 - 0.61805555555555555555e-2 * t4615 + 0.18541666666666666667e-1 * t4618 - 0.18541666666666666667e-1 * t4623;
    let t4732 = t4731 * t274;
    let t4735 = t45 * t1684;
    (t4719, t4721, t4722, t4723, t4725, t4731, t4732, t4735)
}
