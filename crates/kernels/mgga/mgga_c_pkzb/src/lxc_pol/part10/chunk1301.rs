//! MGGA_C_PKZB lxc pol — lxc_pol part 10 (v4rho4_2) CSE chunk 1301/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part10_v4rho4_2_chunk1301<F: Float>(t218: F, t219: F, t25200: F, t675: F, t9187: F, t1066: F, t7350: F, t9194: F, t9198: F, t1843: F, t3515: F, t655: F, t9161: F, t208: F, t25315: F, t25633: F, t25636: F, t25639: F, t25734: F) -> (F, F, F, F, F, F, F, F, F) {
    let t25737 = t218 * t219 * t25200;
    let t25740 = t218 * t675 * t9187;
    let t25744 = t218 * t219 * t1066 * t7350;
    let t25747 = t218 * t675 * t9194;
    let t25750 = t218 * t675 * t9198;
    let t25754 = t218 * t219 * t1843 * t3515;
    let t25758 = t218 * t219 * t655 * t9161;
    let t25762 = t218 * t219 * t208 * t25315;
    let t25764 = 0.39862222222222222223e0 * t25633 - 0.59793333333333333334e0 * t25636 + 0.8969e0 * t25639 + 0.27385555555555555555e0 * t25734 + 0.49294e0 * t25737 - 0.65725333333333333333e0 * t25740 + 0.49294e0 * t25744 - 0.32862666666666666666e0 * t25747 - 0.32862666666666666666e0 * t25750 + 0.24647e0 * t25754 + 0.49294e0 * t25758 + 0.24647e0 * t25762;
    (t25737, t25740, t25744, t25747, t25750, t25754, t25758, t25762, t25764)
}
